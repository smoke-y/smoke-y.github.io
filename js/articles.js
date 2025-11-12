const progPosts = [
    ["zeus", "10/06/2025"],
    ["tamil_gpt", "19/02/2025"],
    ["dl_is_different", "05/02/2025"],
    ["xorwtf", "01/11/2024"],
    ["evolution", "14/10/2024"],
    ["spirit_of_engineering", "08/08/2024"],
    ["uni_first_year", "30/07/2024"],
    ["non_linearity", "20/01/2024"],
]

const miscPosts = [
    ["use_it", "12/11/2025"],
    ["motif", "07/11/2025"],
    ["ibs", "13/07/2025"],
]

function genPosts(id, postsMeta){
    const container = document.getElementById(id);
    postsMeta.forEach(post => {
        const postDiv = document.createElement("li");

        postDiv.innerHTML = `
            <h3><a href="../articles/${post[0]}.html">${post[0]}.html</a><sup>${post[1]}</sup></h3>
        `;
        container.appendChild(postDiv);
    });
}

window.onload = function(){
    genPosts("progPosts", progPosts);
    genPosts("miscPosts", miscPosts);
};
