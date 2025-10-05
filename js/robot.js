let font_width = 8.623; /*Changes based on font*/
function cycle_n(e)
{
    const input_num = e.parentNode.querySelector('input[name="cycle_combo_box"]');
    let value = Number(input_num.value);
    if(e.name === "decrement")
    {
        value--;
    }
    else
    {
        value++;
    }

    if(value < 0)
    {
        input_num.value = 0;
    }
    else
    {
        input_num.value = value;
    }
}

function input_resize(e)
{
    if(e.value.length > 2)
    {
        e.style.width = ((e.value.length+1) * font_width) + 5 + 'px';
    }
    else
    {
        e.style.width = 27.246 + "px";
    }
    return true;
}

function input_resize_var(e)
{
    if(e.value.length > 3)
    {
        e.style.width = ((e.value.length+1) * font_width) + 5 + 'px';
    }
    else
    {
        e.style.width = 27.246 + "px";
    }
    return true;
}

function off_zero(e)
/* off - Out Of Focus */
{
    if(e.value === "")
    {
        e.value = 0;
    }
}

document.querySelector('input[name="cycle_combo_box"]').addEventListener("keydown", omit_keys);
function omit_keys(e)
{
    let temp = e.keyCode;
    if((temp < 96 || temp > 105) && (temp < 48 || temp > 57) && temp !== 46 && temp !== 8 && temp !== 16)
    {
        e.value = 0;
        e.preventDefault();
    }
}
