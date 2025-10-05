function initializeSortable(element) {
    new Sortable(element, {
        group: {
            name: 'menu_default',
            pull: 'clone',
            put: false
        },
        sort: false,
        onClone: (evt) => evt.clone.addEventListener("onkeydown", omit_keys),
        onEnd: createNesting
    });
}

new Sortable(if_state_list, {
    group: {
        name: 'if_state_list',
        pull: (to, _) => to.el.classList.contains("if_statement_field") ? 'clone' : false,
        put: false
    },
    sort: false,
});

new Sortable(sequence_list, {
    group: {
        name: 'sequence',
        pull: true,
        put: true
    },
    swapThreshold: 0.65,
    animation: 150,
    removeOnSpill: true,
});

function createNesting() {
    const container = document.querySelector("#sequence_list");
    const nested_sortables = [].slice.call(container.querySelectorAll('.nested_sortable'));
    const if_statement_fields = [].slice.call(container.querySelectorAll('.if_statement_field'));

    function createSortable(element, group, putCondition) {
        new Sortable(element, {
            group: {
                name: group.name,
                pull: group.pull,
                put: putCondition,
                zIndex: group.zIndex,
                borderTop: 0,
                borderBottom: 0,
            },
            animation: 150,
            swapThreshold: 0.65,
            removeOnSpill: true,
        });
    }

    nested_sortables.forEach((sortable, index) => {
        createSortable(sortable, {
            name: 'nested',
            pull: true,
            zIndex: index,
        }, ['menu_default', 'sequence']);
    });

    if_statement_fields.forEach((sortable, index) => {
        createSortable(sortable, {
            name: 'if_field',
            pull: false,
            zIndex: index,
        }, (to) => (to.el.children.length < 1) ? ['if_state_list'] : false);
    });
}

createNesting();
initializeSortable(menu_list);
initializeSortable(custom_functions_list);
