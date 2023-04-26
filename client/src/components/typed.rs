use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct TypedEffect {}
impl Component for TypedEffect {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <script src="typed.js"></script>
        }
    }
}
