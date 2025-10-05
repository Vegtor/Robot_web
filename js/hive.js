const FieldType = {
    empty : "gray",
    obstacle : "black",
    start : "blue",
    finish : "red",
    star : "yellow",
}

class HexagonalGrid {
    constructor(containerId, numRows, numCols) {
        this.container = document.getElementById(containerId);
        this.stage = new Konva.Stage({
            container: containerId,
            width: this.container.offsetWidth,
            height: this.container.offsetHeight,
        });
        this.layer = new Konva.Layer();
        this.stage.add(this.layer);
        this.numRows = numRows;
        this.numCols = numCols;
        this.hexRadius = this.calculateHexRadius();
        this.hexWidth = Math.sqrt(3) * this.hexRadius;
        this.hexHeight = 2 * this.hexRadius;
        this.hexagonCoords = Array.from({ length: numCols }, () => []);
        this.hexagons = Array.from({ length: numCols }, () => []);

        this.generateGrid();
    }

    // Calculate hexRadius based on the parent container's width and height
    calculateHexRadius() {
        const number_of_rows = this.numRows + 0.5;
        const width = (this.container.offsetWidth * 0.975) / (Math.sqrt(3) * (this.numCols));
        const height = this.container.offsetHeight * 0.95 / (2 * number_of_rows);
        return Math.min(width, height);
    }

    createHexagon(x, y, col, row) {
        const hexagon = new Konva.RegularPolygon({
            x: x,
            y: y,
            sides: 6,
            radius: this.hexRadius,
            fill: FieldType.empty,
            stroke: 'black',
            strokeWidth: 2,
            rotation: 30,
        });

        hexagon.on('mousedown', () => this.handleHexagonClick(row, col));

        hexagon.on('mouseover', (event) => {
            // Check if the left mouse button is pressed
            if (event.evt.buttons === 1) {
                this.handleHexagonHover(row, col);
            }
        });

        this.layer.add(hexagon);

        this.hexagons[col][row] = hexagon;
    }

    handleHexagonClick(row, col) {
        hexClickSubject.next({ row, col });
    }

    handleHexagonHover(row, col) {
        hexHoverPressSubject.next({ row, col });
    }

    changeHexagonColor(x, y, color) {
        const hexagon = this.hexagons[x][y];

        if (hexagon) {
            hexagon.fill(color || 'gray');
            this.layer.draw();
        }
    }

    generateGrid() {
        for (let row = 0; row < this.numRows; row++) {
            for (let col = 0; col < this.numCols; col++) {
                const x = col * this.hexWidth + this.hexRadius;
                const y = row * this.hexHeight + (col % 2) * (this.hexHeight / 2) + this.hexRadius;
                this.createHexagon(x, y, col, row);
                this.hexagonCoords[col][row] = { x, y };
            }
        }

        this.layer.draw();
    }

    getCoordinates(hexCoords) {
        const x = hexCoords.x, y = hexCoords.y;
        if (x >= 0 && x < this.numCols && y >= 0 && y < this.numRows) {
            return this.hexagonCoords[x][y];
        } else {
            return null;
        }
    }
}

class Robot {
    constructor(layer, hexRadius, hexCoords, coords, imageSrc) {
        this.layer = layer;
        this.state = {
            headingDirection: 0,
            hexCoords : {
                x : hexCoords.x,
                y : hexCoords.y,
                theta : 0,
            },
        }

        const imageObj = new Image();
        imageObj.onload = () => {
            this.image = new Konva.Image({
                x: coords.x,
                y: coords.y,
                image: imageObj,
                width: 1.7 * hexRadius,
                height: 1.7 * hexRadius,
                offsetX: 0.85 * hexRadius,
                offsetY: 0.85 * hexRadius,
            });
            this.layer.add(this.image);
            this.layer.draw();
        };
        imageObj.src = imageSrc;
    }

    getKonvaImage() {
        return this.image;
    }
}

let level = {
    dimensions : {
        width: 7,
        height: 7,
    },
    start : {
        x: 0,
        y: 0,
    },
    finish : {
        x: 6,
        y: 6,
    },
    obstacles: [],
    stars: [],
}


const canvas = "map_source";
let hexagonalGrid, robot, finishFlag, starItems = [];
let isMainPage = false;

function removeStar(x, y) {
    let i = 0;
    for (const star of level.stars) {
        if (star.x === x && star.y === y) {
            // Remove from the Konva layer
            const konvaStarToRemove = starItems[i];
            konvaStarToRemove.destroy();
        }
        i++;
    }
}

// Function to handle window resizing
function handleResize() {
    if (hexagonalGrid) {
        hexagonalGrid.stage.destroy();
    }

    hexagonalGrid = new HexagonalGrid(canvas, level.dimensions.height, level.dimensions.width);

    let radius = hexagonalGrid.hexRadius;

    for (const obstacle of level.obstacles) {
        hexagonalGrid.changeHexagonColor(obstacle.x, obstacle.y, FieldType.obstacle);
    }

    starItems = [];
    for (let index = 0; index < level.stars.length; index++) {
        const star = level.stars[index];
        let coords = hexagonalGrid.getCoordinates(star);
        let konvaStar = new Konva.Star({
            x: coords.x,
            y: coords.y,
            numPoints: 6,
            innerRadius: radius * 0.45,
            outerRadius: radius * 0.80,
            fill: 'yellow',
            stroke: 'black',
            strokeWidth: 2,
        });
        hexagonalGrid.layer.add(konvaStar);
        starItems.push(konvaStar);
    }

    if (level.start.x !== -1 && level.start.y !== -1) {
        hexagonalGrid.changeHexagonColor(level.start.x, level.start.y, FieldType.start);
        let coords = hexagonalGrid.getCoordinates(level.start);
        if(isMainPage) {
            robot = new Robot(hexagonalGrid.layer, radius, level.start, coords, '../images/robot.png');
        }
    }
    if (level.finish.x !== -1 && level.finish.y !== -1) {
        hexagonalGrid.changeHexagonColor(level.finish.x, level.finish.y, FieldType.finish);
        let coords = hexagonalGrid.getCoordinates(level.finish);
        if(isMainPage) {
            finishFlag = new Robot(hexagonalGrid.layer, radius, level.finish, coords, '../images/check.png')
        }
    }
}

rxjs.fromEvent(window, 'resize').pipe(
    rxjs.operators.startWith(undefined),

    rxjs.operators.shareReplay(1)
)
.subscribe(handleResize);
