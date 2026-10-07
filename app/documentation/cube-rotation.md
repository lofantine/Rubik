# Rotation des faces du cube

Tout le code décrit ici se trouve dans `src/renderer/lib/cube/class.ts`.

## Le repère

Le cube est centré sur l'origine `(0, 0, 0)`. Pour un cube 3×3, chaque cubelet a des coordonnées dans `{-1, 0, 1}` :

| Axe | `-1`              | `+1`              |
| --- | ----------------- | ----------------- |
| X   | Gauche (orange)   | Droite (rouge)    |
| Y   | Bas (jaune)       | Haut (blanc)      |
| Z   | Arrière (bleu)    | Avant (vert)      |

Pour une taille `n` quelconque, on décale les indices `0 … n-1` de `offset = (n - 1) / 2`. Pour un 2×2 ou un 4×4, les positions sont donc des demi-entiers (`±0.5`, `±1.5`).

Chaque cubelet est un `Three.Mesh` **directement enfant de `root`**. Il n'y a pas de groupe intermédiaire, donc `cubelet.position` est sa vraie position dans le cube.

## Une face = une tranche

Tourner une face revient à tourner tous les cubelets d'une **tranche** perpendiculaire à un axe. Par exemple, la face droite regroupe tous les cubelets avec `x = +1`.

La table `FACE_ROTATIONS` décrit chaque face :

```ts
[Face.Right]: { axis: 'x', side: 'max', sign: -1 },
```

| Champ  | Rôle                                                                     |
| ------ | ------------------------------------------------------------------------ |
| `axis` | Axe autour duquel la tranche tourne                                      |
| `side` | `'min'` = tranche d'indice `0`, `'max'` = tranche d'indice `size - 1`    |
| `sign` | Sens horaire **vu en regardant la face** (voir plus bas)                 |

## Sélection des cubelets

```ts
const target = layer - offset;
if (Math.abs(cubelet.position[axis] - target) > 0.1) return;
```

Pour chaque mouvement, on reparcourt **tous** les cubelets et on garde ceux dont la coordonnée sur l'axe vaut `target` **en ce moment**.

On ne mémorise jamais « quel cubelet est où » : c'est la position actuelle de chaque cubelet qui le dit. Après n'importe quelle suite de mouvements, la sélection est donc toujours juste.

### Pourquoi une tolérance et pas `===` ?

Les rotations sont calculées avec des nombres à virgule. Après des centaines de petits pas, une position qui devrait valoir `1` peut valoir `0.9999999999998`. Avec `===`, ce cubelet ne serait plus sélectionné.

Les positions valides sont espacées de `1`, donc une tolérance de `0.1` ne peut pas confondre deux tranches.

### Pourquoi la sélection ne change pas pendant l'animation ?

Un quart de tour est découpé en `ANIMATION_FRAMES` petits pas, et `rotateFace` est rappelée à chaque pas. Elle refait donc la sélection à chaque fois.

Une rotation autour d'un axe **ne modifie jamais la coordonnée sur cet axe**. En tournant autour de X, seuls Y et Z changent. Les cubelets sélectionnés au premier pas ont toujours la même valeur de `x` au dernier pas, et ce sont exactement les mêmes qui sont sélectionnés.

## La rotation : position + orientation

```ts
const quaternion = new Three.Quaternion().setFromAxisAngle(axisVector, angle);

cubelet.position.applyQuaternion(quaternion);   // 1. orbite
cubelet.quaternion.premultiply(quaternion);     // 2. pivot sur lui-même
```

Un objet 3D a deux propriétés indépendantes : **où il est** (`position`) et **dans quel sens il est tourné** (`quaternion`, ou `rotation`). Un cubelet de Rubik's cube change les deux à chaque mouvement, il faut donc modifier les deux.

### 1. `position.applyQuaternion` : l'orbite

On fait tourner le **vecteur position** autour de l'axe qui passe par le centre du cube. Le cubelet se déplace sur un cercle autour de ce centre.

Exemple avec `U` (face du haut, axe Y, -90°), vu de dessus :

```
         arrière                          arrière
     ┌───┬───┬───┐                    ┌───┬───┬───┐
     │   │   │   │                    │ A │   │   │
     ├───┼───┼───┤        U           ├───┼───┼───┤
     │   │   │   │      ─────►        │   │   │   │
     ├───┼───┼───┤                    ├───┼───┼───┤
     │ A │   │   │                    │   │   │   │
     └───┴───┴───┘                    └───┴───┴───┘
          avant                            avant
  A = (-1, 1, 1)                    A = (-1, 1, -1)
```

Le coin avant-gauche part à l'arrière-gauche : il a **vraiment changé de place**.

### 2. `quaternion.premultiply` : le pivot

Si on ne déplaçait que la position, le cubelet glisserait vers sa nouvelle place **sans tourner** : sa face verte, qui regardait vers l'avant, regarderait toujours vers l'avant. Or sur un vrai cube, l'autocollant vert suit le mouvement et regarde maintenant vers la gauche.

On applique donc **la même rotation** à l'orientation du cubelet. La position et l'orientation tournent du même angle, autour du même axe, et le cubelet se comporte comme un objet rigide attaché à la face.

| Seulement `position` | Seulement `quaternion`        | Les deux                     |
| -------------------- | ----------------------------- | ---------------------------- |
| Bonne place, mauvaises couleurs visibles | Tourne sur place, ne change jamais de tranche | Comportement d'un vrai cube |

### Pourquoi pas `rotation.x += angle` ?

`rotation` contient des **angles d'Euler** : trois angles appliqués dans un ordre fixe (X, puis Y, puis Z). Ajouter un angle à l'un d'eux ne revient pas à « tourner autour de l'axe X du cube » dès que le cubelet a déjà été tourné sur un autre axe.

Exemple : après `R`, faire `rotation.y += angle` ne fait plus tourner autour de l'axe Y du cube, mais autour d'un axe déjà incliné.

Les **quaternions** se composent correctement :

- `premultiply(q)` donne `q × orientation actuelle`, c'est-à-dire « applique `q` **par-dessus** ce qui a déjà été fait, dans le repère du cube ».
- `multiply(q)` ferait l'inverse : il appliquerait `q` dans le repère local du cubelet, déjà tourné, ce qui donnerait un mauvais axe.

### Pourquoi pas un pivot (`Group`) ?

Une autre méthode courante consiste à mettre temporairement la tranche dans un `Group` placé au centre, à tourner ce groupe, puis à remettre les cubelets dans `root` à la fin du mouvement. Le résultat est le même, mais il faut gérer un début et une fin de mouvement.

Ici, `rotateFace` n'a **aucun état**. Chaque appel applique un petit angle et c'est tout. Cela colle à l'animation par petits pas de `animate_move`.

## Le sens de rotation (`sign`)

En Three.js, un angle positif tourne dans le **sens anti-horaire** quand on regarde depuis le côté `+` de l'axe vers l'origine (règle de la main droite).

La notation Rubik's (`R`, `U`, `F`…) désigne un quart de tour **horaire vu en regardant la face**. Donc :

| Face            | On la regarde depuis | Horaire = angle |
| --------------- | -------------------- | --------------- |
| R, U, F (`max`) | le côté `+`          | négatif (`-1`)  |
| L, D, B (`min`) | le côté `-`          | positif (`+1`)  |

Pour un mouvement *prime* (`R'`, `U'`…), on inverse le signe : `direction = prime ? -sign : sign`.

## Déroulement complet d'un mouvement

```
clic sur "R"  /  touche R
   │
   ▼
rotate(Face.Right, false)        → ajoute { face, prime } à la queue
   │                               et lance handle_queue() si rien n'est en cours
   ▼
handle_queue()                   → status = 'animating'
   │                               boucle tant que la queue n'est pas vide
   ▼
animate_move({ Right, false })   → axis 'x', layer size-1, direction -1
   │                               boucle de 0° à 90° par petits pas
   ▼
rotateFace('x', 2, -0.9° en rad) → sélectionne x ≈ 1, orbite + pivot
   │                               (répété ANIMATION_FRAMES fois)
   ▼
queue vide                       → status = 'idle'
```

## Précision

Chaque petit pas introduit une erreur d'arrondi minuscule. Avec des nombres 64 bits, elle reste négligeable : après 1200 quarts de tour enchaînés sur les trois axes, l'écart maximal avec la grille est de l'ordre de `1e-13`, et chaque tranche contient toujours exactement 9 cubelets. La tolérance de `0.1` a donc une marge énorme.
