# Build de l'app

## Commandes

Toutes les commandes se lancent depuis le dossier `app/`.

| Commande            | Rôle                                                  |
| ------------------- | ----------------------------------------------------- |
| `bun start`         | Build complet puis lance l'app Electron               |
| `bun run build`     | Build complet dans `build/` (sans lancer l'app)       |
| `bun run typecheck` | Vérifie les types TypeScript (main + renderer)        |

## Pourquoi plusieurs outils ?

Une app Electron tourne dans **deux process** différents :

- **Main** (`src/main/`) : tourne dans Node.js. Crée la fenêtre, accède au système (fichiers, navigateur externe…).
- **Renderer** (`src/renderer/`) : tourne dans Chromium. C'est la page affichée (React, Tailwind).

Chaque partie a besoin d'un outil adapté :

| Partie          | Outil              | Pourquoi                                                              |
| --------------- | ------------------ | --------------------------------------------------------------------- |
| Main            | `tsc`              | Node sait importer les packages, il suffit de compiler le TS en JS    |
| Renderer (JS)   | `bun build`        | Le navigateur ne sait pas résoudre `import "react"` : il faut un bundler qui regroupe tout dans un seul fichier |
| Renderer (CSS)  | `tailwindcss`      | Génère uniquement les classes Tailwind utilisées dans le code         |
| Renderer (HTML) | `cp`               | L'HTML n'a pas besoin d'être transformé, il est juste copié           |

## Détail de `bun run build`

Le script enchaîne 5 étapes (séparées par `&&` : si une étape échoue, la suite n'est pas lancée).

### 1. Nettoyer

```bash
rm -rf build
```

Supprime l'ancien build pour ne pas garder de fichiers obsolètes.

### 2. Compiler le main

```bash
tsc
```

- Lit `tsconfig.json` (à la racine de `app/`).
- Compile `src/main/**/*.ts` → `build/`.
- Ex : `src/main/index.ts` → `build/index.js`.

`build/index.js` est le point d'entrée d'Electron (champ `"main"` du `package.json`).

### 3. Bundler le renderer

```bash
bun build src/renderer/main.tsx --outdir build/renderer --target browser --minify
```

- Part de `src/renderer/main.tsx` et suit tous les `import` (React, React Router, tes composants…).
- Regroupe tout dans **un seul fichier** : `build/renderer/main.js`.
- `--target browser` : le code tourne dans Chromium, pas dans Node.
- `--minify` : réduit la taille du fichier.

> `bun build` ne vérifie **pas** les types, il les retire simplement. D'où la commande `bun run typecheck`.

### 4. Générer le CSS

```bash
tailwindcss -i src/renderer/index.css -o build/renderer/index.css --minify
```

- `-i` : fichier d'entrée, qui contient `@import "tailwindcss";`.
- Tailwind scanne les fichiers sources (`.tsx`, `.html`…) et ne génère que les classes utilisées.
- `-o` : fichier de sortie, chargé par `index.html`.

### 5. Copier l'HTML

```bash
cp src/renderer/index.html build/renderer/
```

`index.html` contient la `<div id="root">` dans laquelle React s'affiche, et charge `main.js` + `index.css`.

## Résultat

```
build/
├── index.js            ← main     (étape 2)
└── renderer/
    ├── main.js         ← React    (étape 3)
    ├── index.css       ← Tailwind (étape 4)
    └── index.html      ← page     (étape 5)
```

## Lancement : `bun start`

```bash
bun run build && electron .
```

1. Build complet (voir ci-dessus).
2. `electron .` lit le `package.json`, trouve `"main": "build/index.js"` et l'exécute.
3. `build/index.js` crée la fenêtre et charge `build/renderer/index.html`.
4. La page charge `main.js` → React s'affiche dans `#root`.

## Vérification des types : `bun run typecheck`

```bash
tsc --noEmit && tsc -p src/renderer
```

| Commande             | Config utilisée              | Vérifie       |
| -------------------- | ---------------------------- | ------------- |
| `tsc --noEmit`       | `tsconfig.json`              | `src/main/`   |
| `tsc -p src/renderer`| `src/renderer/tsconfig.json` | `src/renderer/` |

Aucun fichier n'est généré (`--noEmit`, et `noEmit: true` dans la config du renderer).
Deux configs car le main tourne dans Node (`NodeNext`) et le renderer dans le navigateur (`DOM`, `jsx`).
