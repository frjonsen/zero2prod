use crate::helpers::spawn_app;
use test_case::test_case;
#[actix_rt::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = app.post_subscriptions(body.into()).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[test_case("name=le%20" => 400; "missing the email")]
#[test_case("email=ursula_le_guin%40gmail.com" => 400; "missing the name")]
#[test_case("" => 400; "missing both name and email")]
#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing(body: &'static str) -> u16 {
    let app = spawn_app().await;
    let response = app.post_subscriptions(body.into()).await;

    response.status().as_u16()
}

#[test_case("name=&email=ursula_le_guin%40.gmail.com")]
#[test_case("name=Ursula&email=")]
#[test_case("name=Ursula&email=definitely-not-an-email")]
#[actix_rt::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid(body: &'static str) {
    let app = spawn_app().await;
    let response = app.post_subscriptions(body.into()).await;

    assert_eq!(
        400,
        response.status().as_u16(),
        "The API did not return a 200 OK"
    );
}
