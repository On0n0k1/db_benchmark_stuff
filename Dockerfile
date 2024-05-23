# FROM mcr.microsoft.com/mssql/server:2019-latest
FROM mcr.microsoft.com/mssql/server:2022-latest

COPY ./backup/Backup.bak /var/opt/backup/Backup.bak

EXPOSE 1433

CMD ["/opt/mssql/bin/sqlservr"]