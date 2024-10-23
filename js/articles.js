const progPosts = [
    ["non_linearity", "20/01/2024"],
    ["uni_first_year", "30/07/2024"],
    ["spirit_of_engineering", "08/08/2024"],
    ["interpretamming", "24/09/2024"],
    ["evolution", "14/10/2024"]
]

const designPosts = [
    ["architecture", "08/08/2024"],
]

function genPosts(id, postsMeta){
    const container = document.getElementById(id);
    postsMeta.forEach(post => {
        const postDiv = document.createElement("li");

        postDiv.innerHTML = `
            <h2 class="red"><a href="../articles/${post[0]}.html" style="color: rgb(173,174,179);">${post[0]}.html</a><sup>${post[1]}</sup></h2>
        `;
        container.appendChild(postDiv);
    });
}

window.onload = function(){
    genPosts("progPosts", progPosts);
    genPosts("designPosts", designPosts);
};