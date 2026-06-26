use wreq::Client;
use wreq_util::Emulation;

pub fn build(follow_redirects: bool) -> wreq::Result<Client> {
    let redirect = if follow_redirects {
        wreq::redirect::Policy::limited(10)
    } else {
        wreq::redirect::Policy::none()
    };

    Client::builder()
        .emulation(Emulation::Chrome149)
        .redirect(redirect)
        .build()
}
