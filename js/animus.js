const rays = [
    [
        { x: 0, y: 1 },
        { x: -1, y: 0 },
        { x: -1, y: -1 },
        { x: 0, y: -1 },
        { x: 1, y: -1 },
        { x: 1, y: 0 }
    ],
    [
        { x: 0, y: 1 },
        { x: -1, y: 1 },
        { x: -1, y: 0 },
        { x: 0, y: -1 },
        { x: 1, y: 0 },
        { x: 1, y: 1 }
    ]
];

const stepCreators = {
    'r': createRotationStep,
    'k': createPositionStep,
};

function createStep(objectState, duration, updateCoordsFn) {
    return rxjs.interval(duration).pipe(
        rxjs.operators.take(1),
        rxjs.operators.map(() => {
            removeStar(objectState.hexCoords.x, objectState.hexCoords.y);
            updateCoordsFn(objectState);
            const newHex = hexagonalGrid.getCoordinates(objectState.hexCoords);
            return { value: { x: newHex.x, y: newHex.y, theta: objectState.hexCoords.theta } };
        })
    );
}

function createRotationStep(objectState, duration) {
    return createStep(objectState, duration, (state) => {
        state.headingDirection = (state.headingDirection + 1) % 6;
        state.hexCoords.theta += 60;
    });
}

function createPositionStep(objectState, duration) {
    return createStep(objectState, duration, (state) => {
        const ray = rays[state.hexCoords.x % 2][state.headingDirection];
        state.hexCoords.x += ray.x;
        state.hexCoords.y += ray.y;
    });
}

function createTween(image, newCoords, duration) {
    return new Konva.Tween({
        node: image,
        x: newCoords.x,
        y: newCoords.y,
        rotation: newCoords.theta,
        duration: duration,
        easing: Konva.Easings.EaseOut,
    });
}

function parseCommand(movableObject, command, animationDuration, message) {
    const animationDurationMilliseconds = animationDuration * 1000;

    rxjs.from(command)
        .pipe(
            rxjs.operators.concatMap((command) => {
                const createStepFn = stepCreators[command];
                if (createStepFn) {
                    return createStepFn(movableObject.state, animationDurationMilliseconds);
                }
            }),
            rxjs.operators.finalize(() => {
                rxjs.timer(animationDurationMilliseconds).subscribe(async () => {
                    await Swal.fire(message);
                    handleResize();
                });
            })
        )
        .subscribe((animationStep) => {
            if (animationStep) {
                const tween = createTween(movableObject.getKonvaImage(), animationStep.value, animationDuration);
                tween.play();

                // Wait for the tween to complete before checking for stars
                rxjs.timer(animationDurationMilliseconds).subscribe(async () => {
                    console.log(starHashMap[`${movableObject.state.hexCoords.x},${movableObject.state.hexCoords.y}`]);
                    if (starHashMap[`${movableObject.state.hexCoords.x},${movableObject.state.hexCoords.y}`] !== undefined) {
                        alert(`Star collected at coordinates (${movableObject.state.hexCoords.x}, ${movableObject.state.hexCoords.y})!`);
                    }
                });
            }
        });
}
