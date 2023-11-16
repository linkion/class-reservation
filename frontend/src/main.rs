use yew::prelude::*;

struct Class {
    id: i32,
    class_name: String,
    max_students: i32,
    registered_students: i32,
    teacher: String,
}

#[function_component(App)]
fn app() -> Html {
    let classes = vec![
        Class {
            id: 1,
            class_name: "Intro to CS II".to_string(),
            max_students: 250,
            registered_students: 47,
            teacher: "Michael Novak".to_string(),
        },
        Class {
            id: 2,
            class_name: "Calculus III".to_string(),
            max_students: 200,
            registered_students: 89,
            teacher: "Jeremiah Heller".to_string(),
        },
    ];
    let classes = classes.iter().map(|class| html! {
        <p key={class.id}>{format!("{} . . . . . . {} . . . . . . {} . . . . . . {}", class.teacher, class.class_name, class.max_students.to_string(), class.registered_students.to_string())}</p>
    }).collect::<Html>();
    html! {
        <>
            <h1>{ "UIUC Class Reservation" }</h1>
            <div>
                <h3>{"Classes: "}</h3>
                {classes}
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}