# Contexto Ativo do Projeto Veloren

## Foco Atual
Implementação do plano de refatoração incremental do arquivo window.rs (1490 linhas) do projeto Veloren.

## Estado Atual
Concluídos com sucesso os Passos 1 e 2 da refatoração: extração dos módulos window_settings.rs (configurações de janela) e input.rs (tipos de entrada). Foi feita também a limpeza de imports não utilizados em window.rs. Preparando-se para implementar o Passo 3: extração do módulo events.rs.

## Decisões Atuais
1. Usar uma abordagem incremental para refatorar window.rs, dividindo-o em etapas menores
2. Reexportar os tipos extraídos para manter a compatibilidade da API pública
3. Verificar a compilação após cada etapa para identificar e resolver problemas rapidamente
4. Avançar para os próximos passos apenas quando o anterior estiver funcionando corretamente
5. Centralizar reexportações para evitar duplicação e conflitos
6. Manter o código limpo, removendo elementos desnecessários após cada passo da refatoração

## Estrutura de Refatoração Proposta
1. **window_settings.rs**: ✅ Configurações de janela (FullscreenMode, WindowSettings, FullScreenSettings)
2. **input.rs**: ✅ Tipos de entrada (MenuInput, AnalogMenuInput, AnalogGameInput)
3. **events.rs**: Definição de eventos (Event e tipos relacionados)
4. **keybindings.rs**: Mapeamento de teclas e exibição (KeyMouse)
5. **window.rs (modificado)**: Implementação principal da janela (Window)

## Plano de Implementação Atualizado
1. **Passo 1**: ✅ Extrair window_settings.rs e verificar compilação
2. **Passo 2**: ✅ Extrair input.rs e verificar compilação
   - Identificar todos os tipos relacionados a input
   - Extrair para o novo arquivo mantendo dependências
   - Atualizar imports e exports em window.rs
   - Verificar compilação e corrigir problemas
   - Limpar imports não utilizados
3. **Passo 3**: Extrair events.rs e verificar compilação
   - Identificar os componentes do enum Event
   - Mover para um novo arquivo mantendo dependências
   - Atualizar imports e reexportações
   - Verificar compilação e corrigir problemas
   - Limpar imports não utilizados se necessário
4. **Passo 4**: Extrair keybindings.rs e verificar compilação
5. **Passo 5**: Refinar window.rs (extrair métodos, organizar imports)

## Lições Aprendidas
- Reexportar os tipos para manter a API pública é essencial para não quebrar o código existente
- Evitar importar e reexportar os mesmos tipos para prevenir conflitos
- Verificar cuidadosamente erros de sintaxe ao fazer alterações no código
- A abordagem incremental facilita a identificação e correção de problemas
- Manter as reexportações centralizadas em um único local para evitar conflitos de definições
- Verificar e remover definições duplicadas ao extrair tipos para novos arquivos
- Remover imports não utilizados após extrair código para novos arquivos

## Próximos Passos
1. Examinar o arquivo window.rs para identificar completamente a estrutura Event e tipos relacionados
2. Criar o arquivo events.rs extraindo esta estrutura e seus tipos relacionados
3. Atualizar window.rs para importar e reexportar corretamente os tipos extraídos
4. Verificar a compilação e resolver problemas se houver
5. Limpar imports não utilizados se necessário
6. Atualizar o banco de memória após o sucesso da implementação do Passo 3

## Métricas de Sucesso
- ✅ Compilação bem-sucedida após Passo 1
- ✅ Compilação bem-sucedida após Passo 2
- ✅ Correção de imports não utilizados
- ⬜ Compilação bem-sucedida após Passo 3
- ⬜ Compilação bem-sucedida após Passo 4
- ⬜ Compilação bem-sucedida após Passo 5
- ⬜ Redução final no tamanho do arquivo window.rs
- ⬜ Separação clara de responsabilidades 