# db_benchmark_stuff
Quick repo for trying out some benchmarking with a few different tools

## Requisitos

 - Docker
 - Docker compose

## Como: Limpar todos conteineres existentes

Isto irá limpar todos os conteineres, imagens, etc do sistema. Não faça isso se tiver algo importante no docker.

```bash
# Ver quais conteineres estão executando
docker container ls
# ou
docker ps

# Ver imagens disponiveis
docker image ls

# Deletar todos conteineres e imagens, etc
docker system prune -a
```

## Iniciar todos conteineres

```bash
docker compose up
```

## Acessar o banco de dados

Accesse o banco de dados como usuário root.

```bash
# Open mssql as follows:
docker ps
# Substitua o ${id} com o id do conteiner db_benchmark_stuff-sqlserver
docker exec -it ${id} /bin/bash
/opt/mssql-tools/bin/sqlcmd -S localhost -U SA -P 'p@ssw0rD!'
```

Cheque os bancos de dados disponiveis.

```SQL
SELECT name
FROM sys.databases;
GO
```

## Restaurar Backup

Mostra a lista de arquivos que o banco de dados usa para armazenar os dados.

```SQL
RESTORE FILELISTONLY FROM DISK = "/var/opt/backup/Backup.bak"
```

É necessário atualizar os caminhos para os arquivos do banco de dados. Por isso,
para este caso, é necessário definir novos caminhos para `AUTOLAC` e `AUTOLAC_LOG`.

```SQL
-- Troque RestoredData pelo nome de banco de dados que preferir
RESTORE DATABASE RestoredData
FROM DISK = "/var/opt/backup/Backup.bak" 
WITH 
MOVE "AUTOLAC" TO "/var/tmp/backup/DATA/AUTOLAC.MDF",
MOVE "AUTOLAC_LOG" TO "/var/tmp/backup/DATA/AUTOLAC.ldf";
GO
```

O único detalhe que deve ser considerado nestes caminhos novos é que devem apontar
para um local em que se tenha permissão para `Read` e `Write`.

## Usar banco de dados

Para as consultas realizadas afetarem o banco de dados liberado pelo backup.

```SQL
-- Troque RestoredData pelo nome de banco de dados que preferir
USE RestoredData;
GO
```

## Comandos

Para checar usuários do banco:
```SQL
SELECT name AS Username, type_desc AS UserType
FROM sys.database_principals
WHERE type IN ('S', 'U', 'G', 'A')
    AND principal_id > 4
ORDER BY name;
GO
```

Para checar as tabelas disponiveis no banco:

```SQL
SELECT name
FROM sys.tables;
GO
```

Para consultar a quantidade de linhas em uma dada tabela:

```SQL
-- Cuidado: Não é uma forma eficiente de calcular tamanho de uma tabela
-- Não use em produção
-- Troque PACIENTE pela tabela que procura checar.
SELECT COUNT(*)
FROM PACIENTE;
GO
```

Para checar as colunas existentes em uma tabela especifica,
incluindo tipos de dados:

```SQL
-- Substitua PACIENTE pela tabela que procura checar.
SELECT COLUMN_NAME, DATA_TYPE, CHARACTER_MAXIMUM_LENGTH
FROM INFORMATION_SCHEMA.COLUMNS
WHERE TABLE_NAME = 'PACIENTE';
GO
```


