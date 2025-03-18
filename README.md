# madruga

**Retry resiliente com preguiça e bom humor, igual o Seu Madruga.**

Um crate para quem precisa de retentativas com backoff — mas com estilo. `madruga` tenta resolver problemas, mas só até onde vale a pena.

> “Não existe trabalho ruim... ruim é ter que trabalhar!” – Seu Madruga

## Recursos

- Estratégias de backoff (fixo, exponencial, jitter)
- Facilidade de uso com `retry_async`
- Mensagens de humor opcionais
- Compatível com `tokio`

## Exemplo básico

```rust
use madruga::{retry_async, RetryStrategy, RetryResult};
use madruga::backoff::Backoff;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let strategy = RetryStrategy::new(5, Backoff::Fixed(Duration::from_secs(1)))
        .with_humor(true)
        .with_language(Language::PtBr);

    let result = madruga_retry!(strategy, |attempt| async move {
        if attempt < 3 {
            Err("Ainda não foi...")
        } else {
            Ok("Agora sim!")
        }
    })
    .await;

    match result {
        RetryResult::Success(val) => println!("Tudo certo: {}", val),
        RetryResult::Failure(e) => println!("Erro final: {}", e),
    }
}

```

## Licença

MIT
