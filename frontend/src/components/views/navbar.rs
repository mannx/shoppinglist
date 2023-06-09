use crate::prelude::*;
use crate::Route;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div id="main_navbar" class="navbar-menu">
                <div class="navbar-start">
                    <a class="navbar-item"><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></a>
                    <a class="navbar-item"><Link<Route> to={Route::Locations}>{"Locations"}</Link<Route>></a>
                    <a class="navbar-item"><Link<Route> to={Route::NewItem}>{"New Item"}</Link<Route>></a>
                </div>
            </div>
        </nav>
    }
}
