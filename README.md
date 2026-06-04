## Variables d'environnement

| Variable | Description | Valeur par défaut | Critique |
|---|---|---|---|
| `ENCRYPT_KEY` | Clé de chiffrement des données | — | ✅ Oui |
| `JWT_SECRET` | Secret pour la signature des tokens JWT | — | ✅ Oui |
| `TOKEN_HASH_KEY` | Clé de hachage des tokens | — | ✅ Oui |
| `DERIVATE_TOKEN_HASH_KEY` | Clé de hachage dérivée des tokens | — | ✅ Oui |
| `CLOUDINARY_CLOUD_NAME` | Identifiant du cloud Cloudinary | — | ✅ Oui |
| `CLOUDINARY_API_KEY` | Clé API Cloudinary | — | ✅ Oui |
| `CLOUDINARY_API_SECRET_KEY` | Clé secrète API Cloudinary | — | ✅ Oui |
| `PORT` | Port d'écoute du serveur | `3000` | Non |
| `CORS_ORIGIN` | Origine autorisée pour le CORS | `*` | Non |
| `BDD` | Nom de la base de données MongoDB | `test-falidex` | Non |
| `CLOUDINARY_MAIN_FOLDER` | Dossier racine Cloudinary | `test_falidex` | Non |
| `MIGRATION_IMG` | Active la migration des images au démarrage (`true`/`false`) | `false` | Non |
| `FIX_RELATION_ID` | Active la correction des IDs de relations au démarrage (`true`/`false`) | `false` | Non |