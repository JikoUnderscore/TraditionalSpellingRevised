import {DICT} from "./dict.js";

const text_box_input = document.getElementById("input");

text_box_input.addEventListener('input', input, false);

let div = document.getElementById('out_put');

function input() {
    div.innerHTML = translation(text_box_input.value);
}


function translation(text) {
    const punct = "'!@#█½-$%’'^&*( '){}[]¿|._-\`/?:;«»‹›—\,“”~";

    for (const pun of punct) {
        text = text.replace(pun, " " + pun);
    }

    text = text.replaceAll("\n", " \n ");

    let ntext = text.toLowerCase().split(" ");

    let last_word = "\n";
    let new_text = "";

    console.log(ntext);
    for (let word of ntext) {
        let got = DICT.get(word)
        if (got === undefined) {
            if (word !== "") {
                if (last_word !== "\n"){
                    new_text += " ";
                }
                new_text += "<span style=\"color: #fa0000;\">" + word + "</span>"; // red are unconfurmed words
            }
        } else {
            if (got === word) {
                if (last_word !== "\n"){
                    new_text += " ";
                }
                new_text += got; // black comfured words with no change of spelling
            } else {
                if (last_word !== "\n"){
                    new_text += " ";
                }
                new_text += "<span style=\"color: #5a65a8;\">" + got + "</span>"; // blue comfurmed word with changed spelling
            }
        }
        last_word = word;
    }


    return new_text;
}


