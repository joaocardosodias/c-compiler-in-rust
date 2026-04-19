
## Exercicio 1

### Enunciado

Considere um navegador Web simples, que tome como entrada uma string de texto em formato HTML e apresente a especificada notação gráfica na tela. O processo de exibição utiliza compilação ou interpretação?

### Resposta

 - Em geral, é interpretação: o navegador lê HTML/CSS e constrói/renderiza diretamente a saída visual sem gerar executável nativo persistente.  
  - Pode haver “traços” de compilação interna (ex.: otimizações de layout/JS JIT), mas no modelo simples do enunciado, classifica melhor como interpretação.

## Exercicio 2

### Enunciado

Ao projetar um compilador, enfrentam-se muitos dilemas. Quais são as cinco qualidades que você, como usuário, considera mais importantes em um compilador que adquire? Essa lista muda quando você é o construtor do compilador? O que sua lista lhe diz, a respeito de um compilador, que você implementaria?

### Resposta

 - Como usuário, uma lista típica forte é: correção, mensagens de erro boas, desempenho do código gerado, tempo de compilação, debuggabilidade/ferramentas.  
  - Como construtor, a prioridade muda para incluir: manutenibilidade, portabilidade, arquitetura modular, testabilidade, robustez de diagnóstico (sem perder correção).  


### Enunciado

Compiladores são usados em muitas circunstâncias diferentes. Que diferenças você poderia esperar nos compiladores projetados para as seguintes aplicações:

a. Um compilador Just-in-time usado para traduzir o código da interface de usuário baixado de uma rede?

### Resposta

 - Priorizaria: latência baixa de compilação, baixo consumo de memória, sandbox/segurança, otimização incremental.  
  - Aceita gerar código menos otimizado para responder rápido.

## Exercicio 3b

### Enunciado

b. Um compilador destinado ao processador embutido usado em um telefone celular?

### Resposta

  - Priorizaria: tamanho do código, eficiência energética, tempo real/previsibilidade, uso cuidadoso de registradores/memória.  
  - Otimizações focadas em footprint e consumo, não só throughput bruto.

## Exercicio 3c

### Enunciado

c. Um compilador usado em um curso de programação introdutório no ensino médio?

### Resposta

 - Priorizaria: mensagens de erro didáticas, recuperação de erro, compilação rápida, simplicidade de linguagem/ferramentas.  
  - Menor foco em otimização agressiva.

## Exercicio 3d

### Enunciado

d. Um compilador usado para criar simulações de túnel de vento executadas em um processador maciçamente paralelo (onde todos os processadores são idênticos)?

### Resposta

 - Priorizaria: otimizações numéricas pesadas, vetorização/paralelização, localidade de memória, escalabilidade.  
  - Tempo de compilação pode ser maior se melhorar bastante tempo de execução.

## Exercicio 3e

### Enunciado

e. Um compilador que visa programas numericamente intensivos para um grande número de máquinas diversas?

### Resposta

  - Priorizaria: portabilidade + backend retargetable, IR rica, autotuning/flags por alvo, testes de corretude numérica.  
  - Normalmente usa pipeline modular com seleção de instruções e alocação de registradores por arquitetura.