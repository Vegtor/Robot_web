let sequence_list = document.getElementById("sequence_list");
let menu_list = document.getElementById("menu_list");
let custom_functions_list = document.getElementById("custom_functions_list");
let if_state_list = document.getElementById("if_state_list");

let clear_button = document.getElementById("btn_clear");
let start_button = document.getElementById("btn_start");
let save_button = document.getElementById("btn_save");

let levelSelect = document.getElementById("level");

function clearSequence() {
    sequence_list.innerHTML = '';
}

function errorAlert(text){
    Swal.fire({
        icon: 'error',
        title: 'Chyba při ukládání příkazu',
        text: text,
    });
}

function processRecursively(element)
{
    const stateProcess = state => state.children[0].getAttribute("data-command");

    let s = "", node;
    let nodes = element.children;
    for (let i = 0; i < nodes.length; i++) {
        node = nodes[i].className;
        /*cyklus*************************************************************/
        if (node.includes("cycle")) {
            let temp = nodes[i].children;
            s += "c" + "(" + temp[0].children[1].children[1].value + "){";
            if(temp[1].children.length > 0) {
                s += processRecursively(temp[1]);
            }
            s += "}"
            /*if podmínka*************************************************************/
        } else if (node.includes("if_podm")) {
            s += "p(";
            s += stateProcess(nodes[i].children[0].children[1]) + "){"
            /* přidat podmíku existence state*/
            if(nodes[i].children[1].children.length > 0) {
                s += processRecursively(nodes[i].children[1]);
            }
            s += "}";
        } else if (node.includes("if_else_podm")) {
            let cond = stateProcess(nodes[i].children[0].children[1]) + "){";
            s += "p(";
            s += cond;
            /* přidat podmíku existence state*/
            /*zpracování if*/
            if(nodes[i].children[1].children.length > 0) {
                s += processRecursively(nodes[i].children[1]);
            }
            s += "}n(" + cond;
            /*zpracování else*/
            if(nodes[i].children[3].children.length > 0) {
                s += processRecursively(nodes[i].children[3]);
            }
            s += "}";
            /*while cyklus*************************************************************/
        } else if (node.includes("while")) {
            s += "d(";
            s += stateProcess(nodes[i].children[0].children[1]) + "){"
            /* přidat podmíku existence state*/
            if (nodes[i].children[1].children.length > 0) {
                s += processRecursively(nodes[i].children[1]);
            }
            s += "}";
            /*jednoduché funkce*************************************************************/
        } else if (node.includes("move")) {
            s += "k";
        } else if (node.includes("rotate")) {
            s += "r";
        } else {
            s += nodes[i].getAttribute("data-command");
        }
    }
    return s;
}

clear_button.addEventListener("click", () => {
    clearSequence();
});

save_button.addEventListener("click", async () => {
    const result = await Swal.fire({
        title: "Nový příkaz",
        text: "Zadejte název příkazu:",
        input: 'text',
        showCancelButton: true
    });

    if (result.dismiss === Swal.DismissReason.cancel || result.dismiss === Swal.DismissReason.backdrop) {
        return;
    }

    let name = result.value.toUpperCase();

    if (name === "") {
        errorAlert('Nezadali jste jméno příkazu!');
        return;
    }

    const parsed_command = processRecursively(sequence_list);

    if (parsed_command === "") {
        errorAlert('Tělo příkazu je prázdné!');
        return;
    }

    try {
        const response = await axios.post('/api/save_command', {
            name: name,
            command: parsed_command,
        });

        const responseData = response.data;

        Swal.fire({
            title: responseData.alert_title,
            text: responseData.message,
            icon: responseData.alert_icon_type,
        });

        if (!responseData.command_saved) {
            return;
        }

        const newCommandElement = `<div class="btn_menu bg-items_color" data-command="${parsed_command}">${name}</div>`;
        custom_functions_list.insertAdjacentHTML("beforeend", newCommandElement);
        clearSequence();
        sequence_list.insertAdjacentHTML("beforeend", newCommandElement);
    } catch (error) {
        errorAlert("Při komunikaci se serverem nastala chyba.");
    }
});

start_button.addEventListener("click", async () => {
    let command = processRecursively(sequence_list);
    let selectedLevel = parseInt(levelSelect.value, 10);

    axios.post('/api/parser', {
        command: command,
        level: selectedLevel,
    }).then(response => {
        if (response.data.error) {
            errorAlert("Při zpracování příkazu nastala chyba.");
            return;
        }
        let message = (response.data.state === "Finished")
            ? {
                icon: 'success',
                title: 'Level úspěšně dokončen',
                text: `Level úspěšně dokončen. Počet kroků: ${response.data.num_of_steps}`,
            }
            : {
                icon: 'info',
                title: 'Level nedokončen',
                text: `Level úspěšně dokončen. Počet kroků: ${response.data.num_of_steps}`,
            };
        parseCommand(robot, response.data.command, 0.4, message);
    });
});

levelSelect.addEventListener("change", function () {
    let selectedLevel = parseInt(levelSelect.value, 10);

    axios.post('/api/level', { level: selectedLevel })
        .then(response => {
            level = response.data;

            handleResize();
        });
});