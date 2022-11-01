use yew::prelude::*;

mod types;
use types::User;

struct UsersComponent {
    users: Vec<User>,
}

impl Component for UsersComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let users = vec![
            User {
                id: 1,
                name: "Bruce".to_string(),
                email: "bruce@wayne.com".to_string(),
            },
            User {
                id: 2,
                name: "Clark".to_string(),
                email: "clark@dailyplanet.com".to_string(),
            },
        ];

        Self { users: users }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Users"}</h1>
                <ul>
                    { for self.users.iter().map(|user| html! {
                        <li>{ &user.name }</li>
                    })}
                </ul>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<UsersComponent>();
}
