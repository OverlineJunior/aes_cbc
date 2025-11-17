#import "@preview/ilm:1.4.1": *

#set text(lang: "pt")

#show: ilm.with(
  title: [Criptografia com AES em modo CBC],
  author: "Francisco Sebastiany Junior",
	abstract: [Relatório sobre a implementação de um programa de CLI em Rust que realiza criptografia e descriptografia utilizando o algoritmo AES em modo CBC.],
  date: datetime.today(),
	date-format: "[day]/[month]/[year]",
	table-of-contents: none,
	chapter-pagebreak: false,
)

= Descrição do Problema

Desenvolver um programa que implemente criptografia e descriptografia utilizando um dos modos de cifra em bloco abaixo:

- _Electronic Code Book_ – ECB
- _Cipher Block Chaining_ – CBC
- _Cipher FeedBack_ – CFB
- _Output FeedBack_ – OFB
- _Stream Cipher Mode_ – SCM
- _Counter Mode_ – CTR

= Solução Proposta

== Escolhas

- Linguagem de Programação: Rust
- Algoritmo de Cifra em Bloco: AES (_Advanced Encryption Standard_)
- Modo de Operação: CBC
- Biblioteca Criptográfica: _soft-aes_
- Biblioteca de CLI: _cliclack_
- Biblioteca de Geração Aleatória: _rand_

== Visão Geral

+ Determinar ação do usuário (criptografar ou descriptografar);

+ Para criptografia:
	- Gerar chave aleatória de 32 bytes (256 bits);
	- Gerar vetor de inicialização (IV) aleatório de 16 bytes (128 bits);
	- Usar modo CBC com padding PKCS7;
	- Concatenar IV e texto cifrado para formar o texto cifrado final;
	- Gerar arquivos de saída com texto cifrado e chave.

+ Para descriptografia:
	- Extrair IV dos primeiros 16 bytes do texto cifrado;
	- Usar modo CBC com padding PKCS7 para descriptografar o restante do texto cifrado;
	- Gerar arquivo de saída com o texto original.

= Resultados

O programa foi implementado com sucesso, permitindo a criptografia e descriptografia de textos utilizando o algoritmo AES em modo CBC. A chave e o vetor de inicialização são gerados aleatoriamente para cada criptografia, garantindo a segurança dos dados.

#image("../images/decrypt.png")
