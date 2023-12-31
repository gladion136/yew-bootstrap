use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
        <div class="row">
            <div class="col-4">
                <nav
                    id="navbar-example3"
                    class="h-100 flex-column align-items-stretch pe-4 border-end position-fixed"
                >
                    <nav class="nav nav-pills flex-column">
                        <a class="nav-link" href="#item-1">{"Item 1"} </a>
                        <nav class="nav nav-pills flex-column">
                            <a class="nav-link ms-3 my-1" href="#item-1-1"
                                >{"Item 1-1"}</a
                            >
                            <a class="nav-link ms-3 my-1" href="#item-1-2"
                                >{"Item 1-2"}</a
                            >
                        </nav>
                        <a class="nav-link" href="#item-2">{"Item 2"}</a>
                        <a class="nav-link" href="#item-3">{"Item 3"}</a>
                        <nav class="nav nav-pills flex-column">
                            <a class="nav-link ms-3 my-1" href="#item-3-1"
                                >{"Item 3-1"}</a
                            >
                            <a class="nav-link ms-3 my-1" href="#item-3-2"
                                >{"Item 3-2"}</a
                            >
                        </nav>
                    </nav>
                </nav>
            </div>

            <div class="col-8">
                <div
                    data-bs-spy="scroll"
                    data-bs-target="#navbar-example3"
                    data-bs-smooth-scroll="true"
                    class="scrollspy-example-2"
                    tabindex="0"
                >
                    <div id="item-1">
                        <h4>{"Item 1"}</h4>
                        <p>
                            {"asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda"}
                        </p>
                    </div>
                    <div id="item-1-1">
                        <h5>{"Item 1-1"}</h5>
                        <p>
                            {"asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda"}
                        </p>
                    </div>
                    <div id="item-1-2">
                        <h5>{"Item 1-2"}</h5>
                        <p>
                            {"asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda"}
                        </p>
                    </div>
                    <div id="item-2">
                        <h4>{"Item 2"}</h4>
                        <p>
                            {"asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda"}
                        </p>
                    </div>
                    <div id="item-3">
                        <h4>{"Item 3"}</h4>
                        <p>
                            {"asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda asd iajsd
                            assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda"}
                        </p>
                    </div>
                    <div id="item-3-1">
                        <h5>{"Item 3-1"}</h5>
                        <p>
                            {"asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda"}
                        </p>
                    </div>
                    <div id="item-3-2">
                        <h5>{"Item 3-2"}</h5>
                        <p>
                            {"asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda asd iajsd
                            assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda
                            asdasoidjasodijjjjjjjj jjjjjjjjjjjjjjjjjjjjjjj
                            jjjjjjjjjjjjjjjjjjjjjjjjjjj sajdoiasod asd oiasdj
                            asd iajsd assssssssssssssssss sadija soda"}
                        </p>
                    </div>
                </div>
            </div>
        </div>
        <script
            src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js"
            integrity="sha384-C6RzsynM9kWDrMNeT87bh95OGNyZPhcTNXj1NW7RuBCsyN/o0jlpcV8Qyq46cDfL"
            crossorigin="anonymous"
        ></script>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
