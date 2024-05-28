#![allow(non_snake_case)]
use crate::{globals, Error};
use serde::{Deserialize, Serialize};
use tiberius::{Client, Config, Row};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[derive(sqlx::FromRow, sqlx::Decode)]
pub struct PacienteTable {
    ID: i32,
    NOME: String,
    ENDERECO: Option<String>,
    BAIRRO: Option<String>,
    CEP: Option<String>,
    CIDADE: Option<String>,
    ESTADO: Option<String>,
    TELEFONE: Option<String>,
    CONTATO: Option<String>,
    CPF: Option<String>,
    IDENTIDADE: Option<String>,
    SEXO: String,
    ESTADOCIVIL: Option<String>,
    GSRH: Option<String>,
    CONVENIO: Option<String>,
    PLANO: Option<String>,
    TITULAR: Option<String>,
    NUMEROCART: Option<String>,
    EMAIL: Option<String>,
    OBSERVACAO: Option<String>,
    CARTEIRA: String,
    CLASSIFICACAO: Option<i32>,
    IDADE: i32,
    CODIGOEXTERNO: Option<String>,
    MATRICULA: Option<String>,
    NUMERO: Option<String>,
    MEDICO: Option<i32>,
    IMPRIMECARTEIRA: String,
    INATIVO: String,
    NACIONALIDADE: Option<String>,
    ORIGEM: Option<String>,
    CHAVE_SLINE: Option<String>,
    NOME_FONETICO: String,
    COR: Option<String>,
    OBS_CLINICA: Option<String>,
    NUMERO_CNS: Option<String>,
    NOME_PAI: Option<String>,
    NOME_MAE: Option<String>,
    COD_IBGE: Option<String>,
    DOCUMENTO_RESP: Option<String>,
    SEGUNDO_NOME: Option<String>,
    CELULAR: Option<String>,
    LOCAL_TRABALHO: Option<String>,
    BLOQUEADO: String,
    RACA: Option<String>,
    ESPECIE: Option<String>,
    PROPRIETARIO: Option<String>,
    COD_EMPRESA: Option<String>,
    COD_UNIDADE: Option<String>,
    EMPRESA: Option<i32>,
    ESTRUTURA_EMPRESA: Option<i32>,
    ETNIA: Option<String>,
    TIPO_LOGRADOURO: Option<String>,
    NUMERO_ENDERECO: Option<i32>,
    COMPLEMENTO: Option<String>,
    EH_DIABETICO: String,
    NATURALIDADE: Option<String>,
    NOME_SOCIAL: Option<String>,
    MOTIVO_BLOQUEIO: Option<String>,
    CNH: Option<String>,
    CATEGORIA_CNH: Option<String>,
    ENVIA_WHATSAPP: Option<String>,
    WHATSAPP: Option<String>,
    NIF: Option<String>,
    COD_ESPECIE: Option<i32>,
    COD_RACA: Option<i32>,
    PASSAPORTE: Option<String>,
}

impl From<PacienteTable> for Paciente {
    fn from(value: PacienteTable) -> Self {
        let ID: i32 = value.ID;
        let NOME: String = value.NOME;
        let ENDERECO: Option<String> = value.ENDERECO;
        let BAIRRO: Option<String> = value.BAIRRO;
        let CEP: Option<String> = value.CEP;
        let CIDADE: Option<String> = value.CIDADE;
        let ESTADO: Option<String> = value.ESTADO;
        let TELEFONE: Option<String> = value.TELEFONE;
        let CONTATO: Option<String> = value.CONTATO;
        let CPF: Option<String> = value.CPF;
        let IDENTIDADE: Option<String> = value.IDENTIDADE;
        let SEXO: String = value.SEXO;
        let ESTADOCIVIL: Option<String> = value.ESTADOCIVIL;
        let GSRH: Option<String> = value.GSRH;
        let CONVENIO: Option<String> = value.CONVENIO;
        let PLANO: Option<String> = value.PLANO;
        let TITULAR: Option<String> = value.TITULAR;
        let NUMEROCART: Option<String> = value.NUMEROCART;
        let EMAIL: Option<String> = value.EMAIL;
        let OBSERVACAO: Option<String> = value.OBSERVACAO;
        let CARTEIRA: String = value.CARTEIRA;
        let CLASSIFICACAO: Option<i32> = value.CLASSIFICACAO;
        let IDADE: i32 = value.IDADE;
        let CODIGOEXTERNO: Option<String> = value.CODIGOEXTERNO;
        let MATRICULA: Option<String> = value.MATRICULA;
        let NUMERO: Option<String> = value.NUMERO;
        let MEDICO: Option<i32> = value.MEDICO;
        let IMPRIMECARTEIRA: String = value.IMPRIMECARTEIRA;
        let INATIVO: String = value.INATIVO;
        let NACIONALIDADE: Option<String> = value.NACIONALIDADE;
        let ORIGEM: Option<String> = value.ORIGEM;
        let CHAVE_SLINE: Option<String> = value.CHAVE_SLINE;
        let NOME_FONETICO: String = value.NOME_FONETICO;
        let COR: Option<String> = value.COR;
        let OBS_CLINICA: Option<String> = value.OBS_CLINICA;
        let NUMERO_CNS: Option<String> = value.NUMERO_CNS;
        let NOME_PAI: Option<String> = value.NOME_PAI;
        let NOME_MAE: Option<String> = value.NOME_MAE;
        let COD_IBGE: Option<String> = value.COD_IBGE;
        let DOCUMENTO_RESP: Option<String> = value.DOCUMENTO_RESP;
        let SEGUNDO_NOME: Option<String> = value.SEGUNDO_NOME;
        let CELULAR: Option<String> = value.CELULAR;
        let LOCAL_TRABALHO: Option<String> = value.LOCAL_TRABALHO;
        let BLOQUEADO: String = value.BLOQUEADO;
        let RACA: Option<String> = value.RACA;
        let ESPECIE: Option<String> = value.ESPECIE;
        let PROPRIETARIO: Option<String> = value.PROPRIETARIO;
        let COD_EMPRESA: Option<String> = value.COD_EMPRESA;
        let COD_UNIDADE: Option<String> = value.COD_UNIDADE;
        let EMPRESA: Option<i32> = value.EMPRESA;
        let ESTRUTURA_EMPRESA: Option<i32> = value.ESTRUTURA_EMPRESA;
        let ETNIA: Option<String> = value.ETNIA;
        let TIPO_LOGRADOURO: Option<String> = value.TIPO_LOGRADOURO;
        let NUMERO_ENDERECO: Option<i32> = value.NUMERO_ENDERECO;
        let COMPLEMENTO: Option<String> = value.COMPLEMENTO;
        let EH_DIABETICO: String = value.EH_DIABETICO;
        let NATURALIDADE: Option<String> = value.NATURALIDADE;
        let NOME_SOCIAL: Option<String> = value.NOME_SOCIAL;
        let MOTIVO_BLOQUEIO: Option<String> = value.MOTIVO_BLOQUEIO;
        let CNH: Option<String> = value.CNH;
        let CATEGORIA_CNH: Option<String> = value.CATEGORIA_CNH;
        let ENVIA_WHATSAPP: Option<String> = value.ENVIA_WHATSAPP;
        let WHATSAPP: Option<String> = value.WHATSAPP;
        let NIF: Option<String> = value.NIF;
        let COD_ESPECIE: Option<i32> = value.COD_ESPECIE;
        let COD_RACA: Option<i32> = value.COD_RACA;
        let PASSAPORTE: Option<String> = value.PASSAPORTE;

        Self {
            ID,
            NOME,
            ENDERECO,
            BAIRRO,
            CEP,
            CIDADE,
            ESTADO,
            TELEFONE,
            CONTATO,
            CPF,
            IDENTIDADE,
            SEXO,
            ESTADOCIVIL,
            GSRH,
            CONVENIO,
            PLANO,
            TITULAR,
            NUMEROCART,
            EMAIL,
            OBSERVACAO,
            CARTEIRA,
            CLASSIFICACAO,
            IDADE,
            CODIGOEXTERNO,
            MATRICULA,
            NUMERO,
            MEDICO,
            IMPRIMECARTEIRA,
            INATIVO,
            NACIONALIDADE,
            ORIGEM,
            CHAVE_SLINE,
            NOME_FONETICO,
            COR,
            OBS_CLINICA,
            NUMERO_CNS,
            NOME_PAI,
            NOME_MAE,
            COD_IBGE,
            DOCUMENTO_RESP,
            SEGUNDO_NOME,
            CELULAR,
            LOCAL_TRABALHO,
            BLOQUEADO,
            RACA,
            ESPECIE,
            PROPRIETARIO,
            COD_EMPRESA,
            COD_UNIDADE,
            EMPRESA,
            ESTRUTURA_EMPRESA,
            ETNIA,
            TIPO_LOGRADOURO,
            NUMERO_ENDERECO,
            COMPLEMENTO,
            EH_DIABETICO,
            NATURALIDADE,
            NOME_SOCIAL,
            MOTIVO_BLOQUEIO,
            CNH,
            CATEGORIA_CNH,
            ENVIA_WHATSAPP,
            WHATSAPP,
            NIF,
            COD_ESPECIE,
            COD_RACA,
            PASSAPORTE,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Paciente {
    ID: i32,
    NOME: String,
    ENDERECO: Option<String>,
    BAIRRO: Option<String>,
    CEP: Option<String>,
    CIDADE: Option<String>,
    ESTADO: Option<String>,
    TELEFONE: Option<String>,
    CONTATO: Option<String>,
    CPF: Option<String>,
    IDENTIDADE: Option<String>,
    SEXO: String,
    ESTADOCIVIL: Option<String>,
    GSRH: Option<String>,
    CONVENIO: Option<String>,
    PLANO: Option<String>,
    TITULAR: Option<String>,
    NUMEROCART: Option<String>,
    EMAIL: Option<String>,
    OBSERVACAO: Option<String>,
    CARTEIRA: String,
    CLASSIFICACAO: Option<i32>,
    IDADE: i32,
    CODIGOEXTERNO: Option<String>,
    MATRICULA: Option<String>,
    NUMERO: Option<String>,
    MEDICO: Option<i32>,
    IMPRIMECARTEIRA: String,
    INATIVO: String,
    NACIONALIDADE: Option<String>,
    ORIGEM: Option<String>,
    CHAVE_SLINE: Option<String>,
    NOME_FONETICO: String,
    COR: Option<String>,
    OBS_CLINICA: Option<String>,
    NUMERO_CNS: Option<String>,
    NOME_PAI: Option<String>,
    NOME_MAE: Option<String>,
    COD_IBGE: Option<String>,
    DOCUMENTO_RESP: Option<String>,
    SEGUNDO_NOME: Option<String>,
    CELULAR: Option<String>,
    LOCAL_TRABALHO: Option<String>,
    BLOQUEADO: String,
    RACA: Option<String>,
    ESPECIE: Option<String>,
    PROPRIETARIO: Option<String>,
    COD_EMPRESA: Option<String>,
    COD_UNIDADE: Option<String>,
    EMPRESA: Option<i32>,
    ESTRUTURA_EMPRESA: Option<i32>,
    ETNIA: Option<String>,
    TIPO_LOGRADOURO: Option<String>,
    NUMERO_ENDERECO: Option<i32>,
    COMPLEMENTO: Option<String>,
    EH_DIABETICO: String,
    NATURALIDADE: Option<String>,
    NOME_SOCIAL: Option<String>,
    MOTIVO_BLOQUEIO: Option<String>,
    CNH: Option<String>,
    CATEGORIA_CNH: Option<String>,
    ENVIA_WHATSAPP: Option<String>,
    WHATSAPP: Option<String>,
    NIF: Option<String>,
    COD_ESPECIE: Option<i32>,
    COD_RACA: Option<i32>,
    PASSAPORTE: Option<String>,
}

impl From<Row> for Paciente {
    fn from(row: Row) -> Self {
        let ID: i32 = row.get("ID").unwrap();
        let NOME: String = row.get::<&str, &str>("NOME").unwrap().to_string();
        let SEXO: String = row.get::<&str, &str>("SEXO").unwrap().to_string();
        let CARTEIRA: String = row.get::<&str, &str>("CARTEIRA").unwrap().to_string();
        let IDADE: i32 = row.get("IDADE").unwrap();
        let IMPRIMECARTEIRA: String = row
            .get::<&str, &str>("IMPRIMECARTEIRA")
            .map(str::to_string)
            .unwrap();
        let INATIVO: String = row
            .get::<&str, &str>("INATIVO")
            .map(str::to_string)
            .unwrap();
        let BLOQUEADO: String = row
            .get::<&str, &str>("BLOQUEADO")
            .map(str::to_string)
            .unwrap();
        let EH_DIABETICO: String = row
            .get::<&str, &str>("EH_DIABETICO")
            .map(str::to_string)
            .unwrap();
        let ENDERECO: Option<String> = row.get::<&str, &str>("ENDERECO").map(str::to_string);
        let BAIRRO: Option<String> = row.get::<&str, &str>("BAIRRO").map(str::to_string);
        let CEP: Option<String> = row.get::<&str, &str>("CEP").map(str::to_string);
        let CIDADE: Option<String> = row.get::<&str, &str>("CIDADE").map(str::to_string);
        let ESTADO: Option<String> = row.get::<&str, &str>("ESTADO").map(str::to_string);
        let TELEFONE: Option<String> = row.get::<&str, &str>("TELEFONE").map(str::to_string);
        let CONTATO: Option<String> = row.get::<&str, &str>("CONTATO").map(str::to_string);
        let CPF: Option<String> = row.get::<&str, &str>("CPF").map(str::to_string);
        let IDENTIDADE: Option<String> = row.get::<&str, &str>("IDENTIDADE").map(str::to_string);
        let ESTADOCIVIL: Option<String> = row.get::<&str, &str>("ESTADOCIVIL").map(str::to_string);
        let GSRH: Option<String> = row.get::<&str, &str>("GSRH").map(str::to_string);
        let CONVENIO: Option<String> = row.get::<&str, &str>("CONVENIO").map(str::to_string);
        let PLANO: Option<String> = row.get::<&str, &str>("PLANO").map(str::to_string);
        let TITULAR: Option<String> = row.get::<&str, &str>("TITULAR").map(str::to_string);
        let NUMEROCART: Option<String> = row.get::<&str, &str>("NUMEROCART").map(str::to_string);
        let EMAIL: Option<String> = row.get::<&str, &str>("EMAIL").map(str::to_string);
        let OBSERVACAO: Option<String> = row.get::<&str, &str>("OBSERVACAO").map(str::to_string);
        let CLASSIFICACAO: Option<i32> = row.get("CLASSIFICACAO");
        let CODIGOEXTERNO: Option<String> =
            row.get::<&str, &str>("CODIGOEXTERNO").map(str::to_string);
        let MATRICULA: Option<String> = row.get::<&str, &str>("MATRICULA").map(str::to_string);
        let NUMERO: Option<String> = row.get::<&str, &str>("NUMERO").map(str::to_string);
        let MEDICO: Option<i32> = row.get("MEDICO");
        let NACIONALIDADE: Option<String> =
            row.get::<&str, &str>("NACIONALIDADE").map(str::to_string);
        let ORIGEM: Option<String> = row.get::<&str, &str>("ORIGEM").map(str::to_string);
        let CHAVE_SLINE: Option<String> = row.get::<&str, &str>("CHAVE_SLINE").map(str::to_string);
        let NOME_FONETICO: String = row
            .get::<&str, &str>("NOME_FONETICO")
            .map(str::to_string)
            .unwrap();
        let COR: Option<String> = row.get::<&str, &str>("COR").map(str::to_string);
        let OBS_CLINICA: Option<String> = row.get::<&str, &str>("OBS_CLINICA").map(str::to_string);
        let NUMERO_CNS: Option<String> = row.get::<&str, &str>("NUMERO_CNS").map(str::to_string);
        let NOME_PAI: Option<String> = row.get::<&str, &str>("NOME_PAI").map(str::to_string);
        let NOME_MAE: Option<String> = row.get::<&str, &str>("NOME_MAE").map(str::to_string);
        let COD_IBGE: Option<String> = row.get::<&str, &str>("COD_IBGE").map(str::to_string);
        let DOCUMENTO_RESP: Option<String> =
            row.get::<&str, &str>("DOCUMENTO_RESP").map(str::to_string);
        let SEGUNDO_NOME: Option<String> =
            row.get::<&str, &str>("SEGUNDO_NOME").map(str::to_string);
        let CELULAR: Option<String> = row.get::<&str, &str>("CELULAR").map(str::to_string);
        let LOCAL_TRABALHO: Option<String> =
            row.get::<&str, &str>("LOCAL_TRABALHO").map(str::to_string);
        let RACA: Option<String> = row.get::<&str, &str>("RACA").map(str::to_string);
        let ESPECIE: Option<String> = row.get::<&str, &str>("ESPECIE").map(str::to_string);
        let PROPRIETARIO: Option<String> =
            row.get::<&str, &str>("PROPRIETARIO").map(str::to_string);
        let COD_EMPRESA: Option<String> = row.get::<&str, &str>("COD_EMPRESA").map(str::to_string);
        let COD_UNIDADE: Option<String> = row.get::<&str, &str>("COD_UNIDADE").map(str::to_string);
        let EMPRESA: Option<i32> = row.get("EMPRESA");
        let ESTRUTURA_EMPRESA: Option<i32> = row.get("ESTRUTURA_EMPRESA");
        let ETNIA: Option<String> = row.get::<&str, &str>("ETNIA").map(str::to_string);
        let TIPO_LOGRADOURO: Option<String> =
            row.get::<&str, &str>("TIPO_LOGRADOURO").map(str::to_string);
        let NUMERO_ENDERECO: Option<i32> = row.get("NUMERO_ENDERECO");
        let COMPLEMENTO: Option<String> = row.get::<&str, &str>("COMPLEMENTO").map(str::to_string);
        let NATURALIDADE: Option<String> =
            row.get::<&str, &str>("NATURALIDADE").map(str::to_string);
        let NOME_SOCIAL: Option<String> = row.get::<&str, &str>("NOME_SOCIAL").map(str::to_string);
        let MOTIVO_BLOQUEIO: Option<String> =
            row.get::<&str, &str>("MOTIVO_BLOQUEIO").map(str::to_string);
        let CNH: Option<String> = row.get::<&str, &str>("CNH").map(str::to_string);
        let CATEGORIA_CNH: Option<String> =
            row.get::<&str, &str>("CATEGORIA_CNH").map(str::to_string);
        let ENVIA_WHATSAPP: Option<String> =
            row.get::<&str, &str>("ENVIA_WHATSAPP").map(str::to_string);
        let WHATSAPP: Option<String> = row.get::<&str, &str>("WHATSAPP").map(str::to_string);
        let NIF: Option<String> = row.get::<&str, &str>("NIF").map(str::to_string);
        let COD_ESPECIE: Option<i32> = row.get("COD_ESPECIE");
        let COD_RACA: Option<i32> = row.get("COD_RACA");
        let PASSAPORTE: Option<String> = row.get::<&str, &str>("PASSAPORTE").map(str::to_string);
        Self {
            ID,
            NOME,
            ENDERECO,
            BAIRRO,
            CEP,
            CIDADE,
            ESTADO,
            TELEFONE,
            CONTATO,
            CPF,
            IDENTIDADE,
            SEXO,
            ESTADOCIVIL,
            GSRH,
            CONVENIO,
            PLANO,
            TITULAR,
            NUMEROCART,
            EMAIL,
            OBSERVACAO,
            CARTEIRA,
            CLASSIFICACAO,
            IDADE,
            CODIGOEXTERNO,
            MATRICULA,
            NUMERO,
            MEDICO,
            IMPRIMECARTEIRA,
            INATIVO,
            NACIONALIDADE,
            ORIGEM,
            CHAVE_SLINE,
            NOME_FONETICO,
            COR,
            OBS_CLINICA,
            NUMERO_CNS,
            NOME_PAI,
            NOME_MAE,
            COD_IBGE,
            DOCUMENTO_RESP,
            SEGUNDO_NOME,
            CELULAR,
            LOCAL_TRABALHO,
            BLOQUEADO,
            RACA,
            ESPECIE,
            PROPRIETARIO,
            COD_EMPRESA,
            COD_UNIDADE,
            EMPRESA,
            ESTRUTURA_EMPRESA,
            ETNIA,
            TIPO_LOGRADOURO,
            NUMERO_ENDERECO,
            COMPLEMENTO,
            EH_DIABETICO,
            NATURALIDADE,
            NOME_SOCIAL,
            MOTIVO_BLOQUEIO,
            CNH,
            CATEGORIA_CNH,
            ENVIA_WHATSAPP,
            WHATSAPP,
            NIF,
            COD_ESPECIE,
            COD_RACA,
            PASSAPORTE,
        }
    }
}

impl Paciente {
    pub async fn tiberius_read(id: i32, config: &Config) -> Result<Self, Error> {
        let tcp = TcpStream::connect(globals::db::ADDRESS)
            .await
            .or_else(Error::tcp_stream)?;
        tcp.set_nodelay(true).unwrap();

        let mut client = Client::connect(config.clone(), tcp.compat_write())
            .await
            .or_else(Error::client_connection)?;
        let query = r#"
            SELECT 
                ID,
                NOME,
                ENDERECO,
                BAIRRO,
                CEP,
                CIDADE,
                ESTADO,
                TELEFONE,
                CONTATO,
                CPF,
                IDENTIDADE,
                SEXO,
                ESTADOCIVIL,
                GSRH,
                CONVENIO,
                PLANO,
                TITULAR,
                NUMEROCART,
                EMAIL,
                OBSERVACAO,
                CARTEIRA,
                CLASSIFICACAO,
                IDADE,
                CODIGOEXTERNO,
                MATRICULA,
                NUMERO,
                MEDICO,
                IMPRIMECARTEIRA,
                INATIVO,
                NACIONALIDADE,
                ORIGEM,
                CHAVE_SLINE,
                NOME_FONETICO,
                COR,
                OBS_CLINICA,
                NUMERO_CNS,
                NOME_PAI,
                NOME_MAE,
                COD_IBGE,
                DOCUMENTO_RESP,
                SEGUNDO_NOME,
                CELULAR,
                LOCAL_TRABALHO,
                BLOQUEADO,
                RACA,
                ESPECIE,
                PROPRIETARIO,
                COD_EMPRESA,
                COD_UNIDADE,
                EMPRESA,
                ESTRUTURA_EMPRESA,
                ETNIA,
                TIPO_LOGRADOURO,
                NUMERO_ENDERECO,
                COMPLEMENTO,
                EH_DIABETICO,
                NATURALIDADE,
                NOME_SOCIAL,
                MOTIVO_BLOQUEIO,
                CNH,
                CATEGORIA_CNH,
                ENVIA_WHATSAPP,
                WHATSAPP,
                NIF,
                COD_ESPECIE,
                COD_RACA,
                PASSAPORTE
            FROM PACIENTE WHERE ID = @P1;"#;
        let stream: tiberius::QueryStream =
            client.query(query, &[&id]).await.or_else(Error::query)?;
        let row: Option<Row> = stream.into_row().await.or_else(Error::into_row)?;
        match row {
            None => Err(Error::NotFound),
            Some(row) => Ok(row.into()),
        }
    }
}
