use leptos::*;
use leptos_router::{use_query_map, Form};

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="bg-white shadow-md p-4">
            <div class="container mx-auto flex items-center justify-between">
                <div class="text-3xl font-bold">{"NWA"}</div>
                <div class="flex items-center gap-2">
                    <input type="text" placeholder="Search Profiles..." class="w-full px-4 py-2 border border-gray-300 rounded-full focus:outline-none" />
                    <img src="nick.png" class="cursor-pointer" />
                </div>
                <nav class="flex items-center gap-4">
                    <a href="#" class="text-gray-600 hover:text-blue-500">{"Home"}</a>
                    <a href="#" class="text-gray-600 hover:text-blue-500">{"About"}</a>
                    <a href="#" class="text-gray-600 hover:text-blue-500">{"Services"}</a>
                    <a href="#" class="text-gray-600 hover:text-blue-500">{"Blog"}</a>
                    <a href="#" class="text-gray-600 hover:text-blue-500">{"Testimonials"}</a>
                    <a href="#" class="text-gray-600 hover:text-blue-500">{"Contact"}</a>
                    <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                        {"Login"}
                    </button>
                </nav>
            </div>
        </header>
    }
}


#[component]
fn Body() -> impl IntoView {
    view! {
        <div class="container mx-auto flex flex-col lg:flex-row items-center justify-between gap-4 p-4">
            <Hero /> 
            <img src="nick.png" class="w-full lg:w-1/2" />
        </div>
    }
}


#[component]
fn Footer() -> impl IntoView {
    view! { 
        <footer class="bg-gray-100 text-gray-600 p-4 mt-4">
            <div class="container mx-auto text-center">
                <p>{"© 2024 NWA. All rights reserved."}</p>
            </div>
        </footer>
    }
}   
 

#[component]
pub fn FormExample() -> impl IntoView {
    view! {
        <div class="container mx-auto my-8 px-4">
            <div class="border-2 border-gray-300 p-6 rounded-lg shadow-lg">
                <h2 class="text-2xl font-semibold mb-6 text-gray-700">Contact Me</h2>
                <form class="space-y-4 text-sm">
                    <div class="flex flex-col mb-4">
                        <label for="firstname" class="mb-2 font-medium text-gray-600">First Name</label>
                        <input type="text" id="firstname" name="firstname" placeholder="Your first name" class="w-full px-4 py-3 border border-gray-300 rounded-md focus:outline-none focus:border-blue-500"/>
                    </div>
                    <div class="flex flex-col mb-4">
                        <label for="lastname" class="mb-2 font-medium text-gray-600">Last Name</label>
                        <input type="text" id="lastname" name="lastname" placeholder="Your last name" class="w-full px-4 py-3 border border-gray-300 rounded-md focus:outline-none focus:border-blue-500"/>
                    </div>
                    <div class="flex flex-col mb-4">
                        <label for="email" class="mb-2 font-medium text-gray-600">Email</label>
                        <input type="email" id="email" name="email" placeholder="Your email address" class="w-full px-4 py-3 border border-gray-300 rounded-md focus:outline-none focus:border-blue-500"/>
                    </div>
                    <div class="flex flex-col mb-4">
                        <label for="number" class="mb-2 font-medium text-gray-600">Phone Number</label>
                        <input type="number" id="number" name="number" placeholder="123-456-7890" class="w-full px-4 py-3 border border-gray-300 rounded-md focus:outline-none focus:border-blue-500"/>
                    </div>
                    <div class="flex flex-col mb-6">
                        <label for="select" class="mb-2 font-medium text-gray-600">Select Option</label>
                        <select id="select" name="select" class="w-full px-4 py-3 border border-gray-300 rounded-md focus:outline-none focus:border-blue-500">
                            <option value="A">Option A</option>
                            <option value="B">Option B</option>
                            <option value="C">Option C</option>
                        </select>
                    </div>
                    <input type="submit" value="Submit" class="w-full bg-blue-500 hover:bg-blue-600 text-white font-bold py-3 px-4 rounded cursor-pointer"/>
                </form>
            </div>
        </div>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <div class="bg-blue-500 text-white p-4 text-center">
            <h1 class="text-4xl font-bold">{"Welcome to NWA"}</h1>
            <p class="mt-2">{"Explore our services and offerings."}</p>
        </div>
    }
}



#[component]
fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen">
            <Header />
            <Hero />
            <Body /> 
            <FormExample />
            <Footer />
        </div>
    }
}


fn main() {
    mount_to_body(|| view! { <App /> });
}



// let (greeting, set_greeting) = create_signal(String::new());

    // let fetch_greeting = move |_| {
    //     let set_greeting = set_greeting.clone();
    //     spawn_local(async move {
    //         if let Ok(response) = reqwest::get("http://127.0.0.1:8000").await {
    //             if let Ok(text) = response.text().await {
    //                 set_greeting(text);
    //             }
    //         }
    //     });
    // };
    // <button on::click(fetch_greeting)>{"Fetch Greeting"}</button>