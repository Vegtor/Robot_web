let add_var_button = document.getElementById("add_var");
let del_var_button = document.getElementById("del_var");

let assignment = document.getElementsByName("type_change");

let list_of_variables = []
let list_of_types = []

let var_type_val = "";
let var_name_val = "";

for(let i = 0; i < assignment.length; i++)
{
    assignment[i].addEventListener("change",() => {
        if (assignment[i].value === "var") {
            assignment[i].parentElement.children[2].disabled = true;
        }
        else {
            assignment[i].parentElement.children[2].disabled = false;
            let index = list_of_variables.indexOf(assignment[i].value);
            if (list_of_types[index] === "num") {
                assignment[i].parentElement.children[2].type = 'number';
            } else if (list_of_types[index] === "bool") {
                assignment[i].parentElement.children[2].type = 'number';
            } else if (list_of_types[index] === "txt") {
                assignment[i].parentElement.children[2].type = 'text';
            }
        }
    });
}

add_var_button.addEventListener("click", async () => {
    await Swal.fire({
        title: "<strong>Přidat proměnnou</strong>",
        html: `
        <div class="flex row h-full w-full justify-center">
            <select name="type" id="type" class="border border-gray-400 rounded-lg h-[50px] shadow-md">
                <option value="bool">Bool</option>
                <option value="num">Číslo</option>
                <option value="txt" selected>Text</option>
            </select>
            <div class="w-[20px]"></div>
            <input type="text" id="var_name" class="border border-gray-400 rounded-lg h-[50px] w-fit shadow-md" placeholder="temp_1">
        </div>
        `,
        showCancelButton: true,
        showConfirmButton: true,
        focusConfirm: true,
        confirmButtonText: '<strong> Uložit </strong>',
        cancelButtonText: '<strong> Zrušit </strong>',
        preConfirm: () => {
            var_name_val = Swal.getPopup().querySelector('#var_name').value;
            var_type_val = Swal.getPopup().querySelector('#type').value;
        },
    }).then(async (result) => {
        if (result.isConfirmed && var_name_val !== "") {
            list_of_variables.push(var_name_val);
            list_of_types.push(var_type_val);
            let opt = document.createElement('option');
            opt.value = var_name_val;
            opt.innerHTML = var_name_val;
            for(let i = 0; i < assignment.length; i++)
            {
                assignment[i].appendChild(opt);
            }
            var_type_val = "";
            var_name_val = "";
            await Swal.fire({
                title: "<strong>Byla přidána proměnná</strong>",
                showConfirmButton: true,
                confirmButtonText: '<strong> OK </strong>',
            });
        }
    });
});

del_var_button.addEventListener("click", async () => {
    await Swal.fire({
        title: "<strong>Smazat proměnnou</strong>",
        html: `
        <div id="variables_list_popup" class="border rounded-lg border-gray-400 h-[80px] w-full overflow-y-scroll sweetscroll shadow-md">
        </div>
        `,
        showCancelButton: true,
        showConfirmButton: true,
        focusConfirm: true,
        confirmButtonText: '<strong> OK </strong>',
        didOpen: () => {
            for (let i = 0; i < list_of_variables.length; i++)
            {
                let temp = document.createElement('div');
                let temp_a = document.createElement('a');
                let temp_a_2 = document.createElement('a');
                let temp_btn = document.createElement('button');

                temp.className = "border-2 flex row w-full h-fit";
                temp_a.text = list_of_variables[i];
                temp_a.className = "w-full h-full";
                temp_a_2.text = list_of_types[i];
                temp_a_2.className = "w-full h-full";
                temp_btn.textContent = "X";
                temp_btn.className = "border-2 w-[40px] h-fit rounded-lg";
                temp_btn.addEventListener("click", delete_variable);
                temp.appendChild(temp_a);
                temp.appendChild(temp_a_2);
                temp.appendChild(temp_btn);

                Swal.getPopup().querySelector("#variables_list_popup").appendChild(temp);
            }

        },
    });
});

function delete_variable(e) {
    let index = list_of_variables.indexOf(e.target.parentElement.children[0].text);
    for(let i = 0; i < assignment.length; i++)
    {
        assignment[i].removeChild(assignment[i].children[index+1]);
    }
    list_of_variables.splice(index,1);
    e.target.parentElement.remove();
    return 0;
}