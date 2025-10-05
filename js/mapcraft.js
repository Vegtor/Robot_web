const hexClickSubject = new rxjs.Subject();
const hexHoverPressSubject = new rxjs.Subject();

let gridArray = Array.from({ length: 7 }, () => Array.from({ length: 7}, () => FieldType.empty));

function makeField(x, y, newField) {
    switch (gridArray[x][y]) {
        case FieldType.obstacle:
            level.obstacles = level.obstacles.filter((field) => field.x !== x || field.y !== y);
            newField = newField === FieldType.obstacle ? FieldType.empty : newField;
            break;
        case FieldType.start:
            level.start = {x: -1, y: -1};
            break;
        case FieldType.finish:
            level.finish = {x: -1, y: -1};
            break;
        case FieldType.star:
            level.stars = level.stars.filter((field) => field.x !== x || field.y !== y);
            newField = newField === FieldType.star ? FieldType.empty : newField;
            break;
    }

    switch (newField) {
        case FieldType.obstacle:
            level.obstacles.push({x, y});
            gridArray[x][y] = FieldType.obstacle;
            hexagonalGrid.changeHexagonColor(x, y, FieldType.obstacle);
            break;
        case FieldType.star:
            level.stars.push({x, y});
            gridArray[x][y] = FieldType.star;
            hexagonalGrid.changeHexagonColor(x, y, FieldType.star);
            break;
        default:
            gridArray[x][y] = FieldType.empty;
            hexagonalGrid.changeHexagonColor(x, y, FieldType.empty);
            break;
    }
}

hexClickSubject.subscribe(({ row, col }) => {
    let x = col, y = row;
    const selectedRadio = document.querySelector('input[name="radioGroup"]:checked').id;
    if (selectedRadio === 'radioObstacle') {
        makeField(x, y, FieldType.obstacle);
    } else if (selectedRadio === 'radioStar') {
        makeField(x, y, FieldType.star);
    } else if (selectedRadio === 'radioStart') {
        let startField = level.start;
        if (startField.x !== -1 || startField.y !== -1) {
            hexagonalGrid.changeHexagonColor(startField.x, startField.y, FieldType.empty);
            gridArray[startField.x][startField.y] = FieldType.empty;
        }

        level.obstacles = level.obstacles.filter((field) => field.x !== x || field.y !== y);
        level.stars = level.stars.filter((field) => field.x !== x || field.y !== y);

        if (gridArray[x][y] === FieldType.finish) {
            level.finish = {x: -1, y: -1};
        }

        hexagonalGrid.changeHexagonColor(x, y, FieldType.start);
        gridArray[x][y] = FieldType.start;
        level.start.x = x;
        level.start.y = y;

    } else if (selectedRadio === 'radioFinish') {
        let finishField = level.finish;
        if (finishField.x !== -1 || finishField.y !== -1) {
            hexagonalGrid.changeHexagonColor(finishField.x, finishField.y, FieldType.empty);
            gridArray[finishField.x][finishField.y] = FieldType.empty;
        }

        level.obstacles = level.obstacles.filter((field) => field.x !== x || field.y !== y)
        level.stars = level.stars.filter((field) => field.x !== x || field.y !== y);

        if (gridArray[x][y] === FieldType.start) {
            level.start = {x: -1, y: -1};
        }

        hexagonalGrid.changeHexagonColor(x, y, FieldType.finish);
        gridArray[x][y] = FieldType.finish;
        level.finish.x = x;
        level.finish.y = y;
    }
});

hexHoverPressSubject.subscribe(({ row, col }) => {
    let x = col, y = row;
    const selectedRadio = document.querySelector('input[name="radioGroup"]:checked').id;

    if (selectedRadio === 'radioObstacle') {
        makeField(x, y, FieldType.obstacle);
    }
});


const buttonDimensions = document.getElementById('buttonDimensions');
const buttonSave = document.getElementById('buttonSaveMap');

buttonDimensions.addEventListener('click', function() {
    const inputRows = document.getElementById('inputRows').value;
    const inputColumns = document.getElementById('inputColumns').value;

    if (!isNaN(inputRows) && !isNaN(inputColumns)) {
        const numRows = parseInt(inputRows, 10);
        const numCols = parseInt(inputColumns, 10);

        level.dimensions.width = numCols;
        level.dimensions.height = numRows;

        level.start = {x: -1, y: -1};
        level.finish = {x: -1, y: -1};

        level.obstacles = [];
        gridArray = Array.from({ length: numCols }, () => Array.from({ length: numRows}, () => FieldType.empty));

        handleResize();
    } else {
        errorAlert('Zadejte prosím validní počet řádků a sloupců.');
    }
});

buttonSave.addEventListener('click', function() {
    if (level.start.x === -1 || level.start.y === -1) {
        Swal.fire({
            icon: 'error',
            title: 'Chybí start',
            text: 'Startovní pole mapy musí být nastaveno',
        })
        return;
    } else if (level.finish.x === -1 || level.finish.y === -1) {
        Swal.fire({
            icon: 'error',
            title: 'Chybí cíl',
            text: 'Cílové pole mapy musí být nastaveno',
        })
        return;
    }

    axios.post('/api/save_level', level)
        .then(response => {
            if (response.data === true) {
                Swal.fire({
                    icon: 'success',
                    title: 'Mapa úspěšně uložena',
                    text: "Mapa byla úspěšně uložena.",
                })
            } else {
                Swal.fire({
                    icon: 'error',
                    title: 'Chyba při ukládání mapy',
                    text: "Mapu se nepovedlo uložit. Zkuste to prosím znovu.",
                })
            }
        });
});
