#![allow(non_snake_case)]
use crate::Error;
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, Mssql, MssqlPool};

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

impl Paciente {
    pub async fn read(id: i32, pool: &MssqlPool) -> Result<Self, Error> {
        let mut conn: PoolConnection<Mssql> = pool.acquire().await.or_else(Error::sqlx)?;
        let paciente: PacienteTable = sqlx::query_as(
            r#"
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
         FROM PACIENTE WHERE ID = @P1;"#,
        )
        .bind(id)
        .fetch_one(&mut conn)
        .await
        .or_else(Error::sqlx)?;
        let paciente: Paciente = paciente.into();
        Ok(paciente)
    }

    pub async fn read_many(
        quantity: i32,
        offset: i32,
        pool: &MssqlPool,
    ) -> Result<Vec<Paciente>, Error> {
        let mut conn: PoolConnection<Mssql> = pool.acquire().await.or_else(Error::sqlx)?;
        let pacientes: Vec<PacienteTable> = sqlx::query_as(
            r#"
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
         FROM PACIENTE 
         ORDER BY ID ASC
         OFFSET @P1 ROWS
         FETCH NEXT @P2 ROWS ONLY;"#,
        )
        .bind(offset)
        .bind(quantity)
        .fetch_all(&mut conn)
        .await
        .or_else(Error::sqlx)?;
        let pacientes: Vec<Paciente> = pacientes.into_iter().map(Into::into).collect();
        Ok(pacientes)
    }
}
