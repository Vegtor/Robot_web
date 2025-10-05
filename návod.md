# Návod

Pokud chcete hrát, musíte se registrovat vyplněním formuláře.

## Hra
Po úspěšném přihlášení jste přesměrováni na hrací plátno. V levém bloku jsou umístěny příkazy, ze kterých si můžete vytvářet funkce.

### Panel funkcí
-   <b>Cyklus</b> - provede zadanou sekvenci příkazů tolikrát, kolikrát uvedete.
-   <b>Pokud</b> - podmínka. Například příkazy zadané do těla Pokud(zeď) se provedou v případě, že robot směřuje do zdi.
-   <b>Pokud nebo</b> - podmínka. Například příkazy zadané do těla Pokud(zeď) se provedou v případě, že robot směřuje do zdi, ale pokud do zdi nesměřuje, provedou se příkazy v těle Nebo.
-   <b>Dokud neni</b> -cyklus s podmínkou. Provádí se příkazy v těle, dokud není splněná zadaná podmínka.
-   <b>Krok</b> - posune robota o jedno políčko vpřed směrem, kam je nasměrován.
-   <b>Rotace</b> - Otočí robota o jeden směr po směru hodinových ručiček.

### Panel podmínek

-   <b>Sever</b> - pokud robot směřuje nahoru.
-   <b>Jih</b> - pokud robot směřuje dolu.
-   <b>Severozápad</b> - pokud robot směřuje mírně nahoru a doleva.
-   <b>Severovýchod</b> - pokud robot směřuje mírně nahoru a doprava.
-   <b>Jihozápad</b> - pokud robot směřuje mírně dolu a doleva.
-   <b>Jihovýchod</b> - pokud robot směřuje mírně dolu a doprava.
-   <b>Zeď</b> - pokud robot směřuje do zdi.

Do prostředního bloku se přesunují příkazy, které chcete vykonat. Přetahují se na základě podrž myš, přesuň a pusť.

V pravém bloku je vidět mapa. 

-   <b>Šedé políčko</b> - prázdné políčko, na které může robot vstoupit.
-   <b>Černé políčko</b> - překážka = políčko, na které nemůže robot vstoupit.
-   <b>Modré políčko</b> - start, na kterém je robot umístěn.
-   <b>Červené políčko</b> - cíl, kam se musíte dostat pro splnění levelu.
-   <b>Šedé políčko s hvězdou</b> - prázdné políčko, na které může robot vstoupit a sebrat hvězdu, která odečítá 3 kroky.

Dole pod tímto blokem jsou umístěna 3 tlačítka:
-   <b>Vykonej</b> - vykoná Vámi zadanou funkci, pokud ale nedosáhnete cíle, robot se vrátí zpátky na start.
-   <b>Vyčisti</b> - vyčistí prostřední blok s příkazy.
-   <b>Ulož funkci</b> - uloží Vámi zadanou funkci.

## Editor
V editoru máte možnost vytvořit si vlastní mapu. Nahoře jsou textová pole, kam můžete vložit pouze čísla pro určení velikosti mapy. Po zadání těchto čísel musíte kliknout na tlačítko <b>nastav</b>, aby přenastavil rozměry mapy . Dále si zaškrtnete, co chcete do mapy přidat (překážka/start/cíl/hvězda). Každá mapa musí obsahovat start, který tam může být pouze jednou a stejně tak cíl. Po dokončení Vámi vytvořené mapy stisknete uprostřed dole tlačítko <b>uložit mapu</b> a Vaše mapa se uloží a budete si ji moci zahrát.