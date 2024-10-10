import { base } from '$app/paths';

export async function load({ fetch }) {

    const url = base + "/hello?name=PageLoadFunction";
    const opts = {
        method: "get",
        headers: { "Content-Type": "application/json" }
    };
    const response = await fetch(url, opts);
    const hello = await response.text();
    console.log("Hello:", hello);

    return {
        hello
    };

}
