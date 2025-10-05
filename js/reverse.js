function getBlock(s)
{
    let result = 1;
    let counter = 0;
    for (let i = 0; i < s.length; i++)
    {
        if(s.charAt(i) === '{')
        {
            counter ++;
        }
        if(s.charAt(i) === '}')
        {
            counter --;
            if (counter === 0)
            {
                result += i;
                break;
            }
        }
    }
    return result;
}

function createHTML(type, state, block)
{
    let result= "";
    switch (type) {
        case 0:
            result += "                <div class=\"btn_menu bg-items_color cycle w-full h-fit justify-end\">\n" +
                "                    <div class=\"w-full h-fit flex flex-row justify-end\">\n" +
                "                        <a class=\"mr-2\">Opakuj</a>\n" +
                "                        <div class=\"w-fit h-fit custom-number-input flex flex-row justify-end\">\n" +
                "                            <button name=\"increment\" class=\"h-fit w-[15px] m-0 font-bold text-white\" onclick=\"cycle_n(this)\">+</button>\n" +
                "                            <input name=\"cycle_combo_box\" type=\"number\" value=\"" + state + "\" min=\"0\" class=\"min-w-[27.246px] w-[27.246px] h-fit text-center m-0 cycle_combo_box text-black rounded-[6px]\" oninput=\"input_resize(this)\" onkeydown=\"omit_keys(this)\" onfocusout=\"off_zero(this)\">\n" +
                "                            <button name=\"decrement\" class=\"h-fit w-[15px] m-0 font-bold text-white\" onclick=\"cycle_n(this)\">-</button>\n" +
                "                        </div>\n" +
                "                    </div>\n" +
                "                    <div class=\"nested_sortable bg-nested_color border-nested_color\">\n" + block +
                "                    </div>\n" +
                "                </div>";
            break;
        case 1:
            result += "<div class=\"btn_menu bg-items_color if_podm w-full h-fit\">\n" +
                "                    <div class=\"w-full h-fit flex flex-row justify-end\">\n" +
                "                        <a class=\"mr-2\">Pokud </a>\n" +
                "                        <div class=\"if_statement_field bg-nested_color border-nested_color \"> " + state + " </div>\n" +
                "                    </div>\n" +
                "                    <div class=\"nested_sortable bg-nested_color border-nested_color\"> " + block + "</div>\n"
                "                </div>";
            break;
        case 2:
            let blocks = block.split("&");
            result += "                <div class=\"btn_menu bg-items_color if_else_podm w-full h-fit\">\n" +
                "                    <div class=\"w-full h-fit flex flex-row justify-end\">\n" +
                "                        <a class=\"mr-2\">Pokud </a>\n" +
                "                        <div class=\"if_statement_field bg-nested_color border-nested_color \"> " + state + " </div>\n" +
                "                    </div>\n" +
                "\n" +
                "                    <div class=\"nested_sortable bg-nested_color border-nested_color\"> \n" + blocks[0] +
                "                    </div>\n" +
                "                    <div class=\"w-full h-fit flex flex-row justify-end\">\n" +
                "                        <div class=\"w-full h-fit flex justify-end\">\n" +
                "                            <a>nebo</a>\n" +
                "                        </div>\n" +
                "                    </div>\n" +
                "                    <div class=\"nested_sortable bg-nested_color border-nested_color\"> \n" + blocks[1] +
                "                    </div>\n" +
                "                </div>";
            break;
        case 3:
            result += "                <div class=\"btn_menu bg-items_color while w-full h-fit\">\n" +
                "                    <div class=\"w-full h-fit flex flex-row justify-end\">\n" +
                "                        <a class=\"mr-2\">Dokud není</a>\n" +
                "                        <div class=\"if_statement_field bg-nested_color border-nested_color \"> " + state + " </div>\n" +
                "                    </div>\n" +
                "                    <div class=\"nested_sortable bg-nested_color border-nested_color\"> \n" + block +
                "                    </div>\n" +
                "                </div>";
            break;
        case 4:
            result += "<div class=\"btn_menu bg-items_color move\">Krok\n" +
                "                </div>";
            break;
        case 5:
            result += "<div class=\"btn_menu bg-items_color rotate\">Rotace\n" +
                "                </div>";
            break;
        case 11:
            result += "<div class=\"btn_menu bg-state_color north\">Sever\n" +
                "                </div>";
            break;
        case 12:
            result += "<div class=\"btn_menu bg-state_color south\">Jih\n" +
                "                </div>";
            break;
        case 13:
            result += "<div class=\"btn_menu bg-state_color north_west\">Severozápad\n" +
                "                </div>";
            break;
        case 14:
            result += "<div class=\"btn_menu bg-state_color north_east\">Severovýchod\n" +
                "                </div>";
            break;
        case 15:
            result += "<div class=\"btn_menu bg-state_color south_west\">Jihozápad\n" +
                "                </div>";
            break;
        case 16:
            result += "<div class=\"btn_menu bg-state_color south_east\">Jihovýchod\n" +
                "                </div>";
            break;
        case 17:
            result += "<div class=\"btn_menu bg-state_color south_east\">Zeď\n" + "</div>";
            break;
        default:
            break;
    }
    return result;
}

function getState(str)
{
    let result;
    switch(str)
    {
        case "s":
            result = 11;
            break;
        case "j":
            result = 12;
            break;
        case "q":
            result = 13;
            break;
        case "e":
            result = 14;
            break;
        case "y":
            result = 15;
            break;
        case "v":
            result = 16;
            break;
        case "z":
            result = 17;
            break;
    }
    return result;
}

function deprocessRecursively(funString)
{
    let result = "";
    let s = funString;
    let block = "";
    let index;
    let n = "";
    let i = 0;
    let state = "";
    while (s !== "") {
        let temp = s.charAt(0);
        switch (temp)
        {
            case "c":
                let number = s.slice(0, s.indexOf(")"));
                number = number.slice(number.indexOf("(") + 1);
                s = s.slice(3+number.length);
                index = getBlock(s);
                block = s.slice(0,index-1);
                s = s. slice(index);
                if(block.length>1)
                {
                    block = block.slice(1);
                    block = deprocessRecursively(block);
                }
                result += createHTML(0, number, block);
                break;
            case "p":
                state = s.slice(0, s.indexOf(")"));
                state = state.slice(state.indexOf("(") + 1);
                s = s.slice(3+state.length);
                n = createHTML(getState(state), "", "");
                index = getBlock(s);
                block = s.slice(0,index-1);
                s = s. slice(index);
                if(block.length>1)
                {
                    block = block.slice(1);
                    block = deprocessRecursively(block);
                }
                if(s.charAt(0) === "n")
                {
                    s = s.slice(4);
                    index = getBlock(s);
                    let blockN = s.slice(0,index-1);
                    s = s. slice(index);
                    if(blockN.length>1)
                    {
                        blockN = blockN.slice(1);
                        blockN = deprocessRecursively(blockN);
                    }
                    result += createHTML(2, n, block + "&" + blockN);
                    break;
                }
                result += createHTML(1, n, block);
                break;
            case "d":
                state = s.slice(0, s.indexOf(")"));
                state = state.slice(state.indexOf("(") + 1);
                s = s.slice(3+state.length);
                n = createHTML(getState(state), "", "");
                index = getBlock(s);
                block = s.slice(0,index-1);
                s = s. slice(index);
                if(block.length>1)
                {
                    block = block.slice(1);
                    block = deprocessRecursively(block);
                }
                result += createHTML(3, n, block);
                break;
            case "k":
                result += createHTML(4, "", "");
                s = s.slice(1);
                break;
            case "r":
                result += createHTML(5, "", "");
                s = s.slice(1);
                break;
            default:
                break;
        }
    }
    return result;
}

function deleteFunction(command) {
    Swal.fire({
        title: 'Opravdu chcete smazat funkci ' + command + " ?",
        text: 'Opravdu chcete smazat funkci ' + command + " ? Tato operace je nenávratná.",
        icon: 'warning',
        showCancelButton: true,
        confirmButtonColor: '#3085d6',
        cancelButtonColor: '#d33',
        confirmButtonText: 'Smazat funkci!'
    }).then((result) => {
        if (result.isConfirmed) {
            axios.post('/api/delete_function', { command: command })
                .then(() => {
                    Swal.fire({
                        icon: 'success',
                        title: 'Funkce byla úspěšně smazána',
                        showConfirmButton: true,
                        timer: 1500
                    }).then(() => {
                        // Reload the page after the SweetAlert
                        location.reload();
                    });
                })
        }
    });
}
