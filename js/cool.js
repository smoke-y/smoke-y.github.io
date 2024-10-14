const coolStuff = [
    ["https://youtu.be/u0OY1RDe8Yg?si=HIRmPI7__m_LoiRl", "the secret of psalm 46", "My favourite game dev talk"],
    ["https://youtu.be/SqFu5O-oPmU?si=L8rA4ir9V2egaiw5", "video games and the human condition", "My 2nd favourite game dev talk(All of J.blow talks are amazing)"],
    ["https://www.unqualified-reservations.org/#archive", "unqualified-reservations", "Approaching politics as an engineering problem"],
    ["https://youtu.be/9iUvuaChDEg?si=N3bgOTM_45CMPEZd", "geohot's disstrack", "Jailbreak PS3 -> gets sued by sony -> release disstrack on yt(My idol fr)"],
    ["https://youtu.be/A2dxjOjWHxQ?si=2Y1lZHnpnoWcLEJa", "handmade hero", "2nd most influential software movement after open-source"],
]
window.onload = function(){
    const container = document.getElementById("cool");
    coolStuff.forEach(post => {
        const postDiv = document.createElement("li");

        postDiv.innerHTML = `
            <h2 class="headingStart"><a href="${post[0]}" rel="noopener noreferrer">${post[1]}</a></h2>
            <h3 style="display: inline">${post[2]}</h3>
        `;
        container.appendChild(postDiv);
    });
};