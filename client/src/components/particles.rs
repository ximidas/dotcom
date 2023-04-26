use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct ParticlesEffect {}
impl Component for ParticlesEffect {
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
            <script src="particles.js"></script>
        }
    }
}
