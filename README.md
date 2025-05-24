# Structure 5/23/25 Angelax
* ***/Architecture.md/Structure.md*** *
--------------------------------------------
```rust
.
├── Architecture
│   └── Structure.md
├── CHANGELOG.md
├── CLI
│   ├── QUICKSTART.md
│   ├── angela
│   │   ├── __init__.py
│   │   ├── __main__.py
│   │   ├── api
│   │   │   ├── __init__.py
│   │   │   ├── ai.py
│   │   │   ├── cli.py
│   │   │   ├── context.py
│   │   │   ├── execution.py
│   │   │   ├── generation.py
│   │   │   ├── intent.py
│   │   │   ├── interfaces.py
│   │   │   ├── monitoring.py
│   │   │   ├── review.py
│   │   │   ├── safety.py
│   │   │   ├── shell.py
│   │   │   ├── toolchain.py
│   │   │   └── workflows.py
│   │   ├── cli
│   │   │   └── __init__.py
│   │   ├── components
│   │   │   ├── ai
│   │   │   │   ├── __init__.py
│   │   │   │   ├── analyzer.py
│   │   │   │   ├── client.py
│   │   │   │   ├── confidence.py
│   │   │   │   ├── content_analyzer.py
│   │   │   │   ├── content_analyzer_extensions.py
│   │   │   │   ├── enhanced_prompts.py
│   │   │   │   ├── file_integration.py
│   │   │   │   ├── intent_analyzer.py
│   │   │   │   ├── parser.py
│   │   │   │   ├── prompts.py
│   │   │   │   └── semantic_analyzer.py
│   │   │   ├── cli
│   │   │   │   ├── __init__.py
│   │   │   │   ├── docker.py
│   │   │   │   ├── files.py
│   │   │   │   ├── files_extensions.py
│   │   │   │   ├── generation.py
│   │   │   │   ├── main.py
│   │   │   │   ├── utils
│   │   │   │   │   ├── __init__.py
│   │   │   │   │   └── async_helpers.py
│   │   │   │   └── workflows.py
│   │   │   ├── context
│   │   │   │   ├── __init__.py
│   │   │   │   ├── enhanced_file_activity.py
│   │   │   │   ├── enhancer.py
│   │   │   │   ├── file_activity.py
│   │   │   │   ├── file_detector.py
│   │   │   │   ├── file_resolver.py
│   │   │   │   ├── history.py
│   │   │   │   ├── manager.py
│   │   │   │   ├── preferences.py
│   │   │   │   ├── project_inference.py
│   │   │   │   ├── project_state_analyzer.py
│   │   │   │   ├── semantic_context_manager.py
│   │   │   │   └── session.py
│   │   │   ├── execution
│   │   │   │   ├── __init__.py
│   │   │   │   ├── adaptive_engine.py
│   │   │   │   ├── engine.py
│   │   │   │   ├── error_recovery.py
│   │   │   │   ├── filesystem.py
│   │   │   │   ├── hooks.py
│   │   │   │   ├── rollback.py
│   │   │   │   └── rollback_commands.py
│   │   │   ├── generation
│   │   │   │   ├── __init__.py
│   │   │   │   ├── architecture.py
│   │   │   │   ├── context_manager.py
│   │   │   │   ├── documentation.py
│   │   │   │   ├── engine.py
│   │   │   │   ├── frameworks.py
│   │   │   │   ├── models.py
│   │   │   │   ├── planner.py
│   │   │   │   ├── refiner.py
│   │   │   │   └── validators.py
│   │   │   ├── intent
│   │   │   │   ├── __init__.py
│   │   │   │   ├── complex_workflow_planner.py
│   │   │   │   ├── enhanced_task_planner.py
│   │   │   │   ├── models.py
│   │   │   │   ├── planner.py
│   │   │   │   └── semantic_task_planner.py
│   │   │   ├── interfaces
│   │   │   │   ├── __init__.py
│   │   │   │   ├── execution.py
│   │   │   │   └── safety.py
│   │   │   ├── monitoring
│   │   │   │   ├── __init__.py
│   │   │   │   ├── background.py
│   │   │   │   ├── network_monitor.py
│   │   │   │   ├── notification_handler.py
│   │   │   │   └── proactive_assistant.py
│   │   │   ├── review
│   │   │   │   ├── __init__.py
│   │   │   │   ├── diff_manager.py
│   │   │   │   └── feedback.py
│   │   │   ├── safety
│   │   │   │   ├── __init__.py
│   │   │   │   ├── adaptive_confirmation.py
│   │   │   │   ├── classifier.py
│   │   │   │   ├── confirmation.py
│   │   │   │   ├── preview.py
│   │   │   │   └── validator.py
│   │   │   ├── shell
│   │   │   │   ├── __init__.py
│   │   │   │   ├── advanced_formatter.py
│   │   │   │   ├── angela.bash
│   │   │   │   ├── angela.tmux
│   │   │   │   ├── angela.zsh
│   │   │   │   ├── angela_enhanced.bash
│   │   │   │   ├── angela_enhanced.zsh
│   │   │   │   ├── completion.py
│   │   │   │   ├── formatter.py
│   │   │   │   └── inline_feedback.py
│   │   │   ├── toolchain
│   │   │   │   ├── __init__.py
│   │   │   │   ├── ci_cd.py
│   │   │   │   ├── cross_tool_workflow_engine.py
│   │   │   │   ├── docker.py
│   │   │   │   ├── enhanced_universal_cli.py
│   │   │   │   ├── git.py
│   │   │   │   ├── package_managers.py
│   │   │   │   ├── test_frameworks.py
│   │   │   │   └── universal_cli.py
│   │   │   ├── utils
│   │   │   │   ├── __init__.py
│   │   │   │   ├── enhanced_logging.py
│   │   │   │   └── logging.py
│   │   │   └── workflows
│   │   │       ├── __init__.py
│   │   │       ├── manager.py
│   │   │       └── sharing.py
│   │   ├── config.py
│   │   ├── constants.py
│   │   ├── core
│   │   │   ├── __init__.py
│   │   │   ├── events.py
│   │   │   └── registry.py
│   │   ├── orchestrator.py
│   │   └── utils
│   │       ├── async_utils.py
│   │       ├── command_utils.py
│   │       └── logging.py
│   ├── pyproject.toml
│   ├── pytest.ini
│   └── scripts
│       ├── install-quick.sh
│       ├── install.sh
│       └── uninstall.sh
├── CONTRIBUTING.md
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── PLANS
│   ├── Cargo.toml-plan.md
│   ├── PLAN.MD
│   ├── STEPS.md
│   └── crates
│       ├── angelax-allocator
│       │   └── src
│       │       └── lib-plan.md
│       ├── angelax-auth
│       │   └── src
│       │       ├── jwt
│       │       │   ├── algorithms-plan.md
│       │       │   ├── claims-plan.md
│       │       │   ├── mod-plan.md
│       │       │   ├── token-plan.md
│       │       │   └── validation-plan.md
│       │       ├── lib-plan.md
│       │       ├── oauth
│       │       │   ├── client-plan.md
│       │       │   ├── flows-plan.md
│       │       │   ├── mod-plan.md
│       │       │   ├── providers-plan.md
│       │       │   └── server-plan.md
│       │       ├── password
│       │       │   ├── argon2-plan.md
│       │       │   ├── bcrypt-plan.md
│       │       │   ├── hash-plan.md
│       │       │   ├── mod-plan.md
│       │       │   └── scrypt-plan.md
│       │       ├── rbac
│       │       │   ├── evaluation-plan.md
│       │       │   ├── mod-plan.md
│       │       │   ├── permission-plan.md
│       │       │   ├── policy-plan.md
│       │       │   └── role-plan.md
│       │       ├── security
│       │       │   ├── audit-plan.md
│       │       │   ├── csrf-plan.md
│       │       │   ├── headers-plan.md
│       │       │   ├── mod-plan.md
│       │       │   └── xss-plan.md
│       │       └── session
│       │           ├── cookie-plan.md
│       │           ├── database-plan.md
│       │           ├── memory-plan.md
│       │           ├── mod-plan.md
│       │           └── redis-plan.md
│       ├── angelax-cli
│       │   └── src
│       │       ├── commands
│       │       ├── dev_server
│       │       ├── generator
│       │       ├── main-plan.md
│       │       ├── templates
│       │       └── utils
│       ├── angelax-common
│       │   └── src
│       │       ├── constants-plan.md
│       │       ├── error-plan.md
│       │       ├── lib-plan.md
│       │       ├── traits-plan.md
│       │       └── types-plan.md
│       ├── angelax-config
│       │   └── src
│       │       ├── lib-plan.md
│       │       ├── loader-plan.md
│       │       ├── macros-plan.md
│       │       ├── source-plan.md
│       │       └── validation-plan.md
│       ├── angelax-core
│       │   └── src
│       │       ├── config
│       │       │   ├── app_config-plan.md
│       │       │   ├── mod-plan.md
│       │       │   └── settings-plan.md
│       │       ├── error
│       │       │   ├── error-plan.md
│       │       │   ├── handler-plan.md
│       │       │   ├── mod-plan.md
│       │       │   └── recovery-plan.md
│       │       ├── lib-plan.md
│       │       ├── middleware
│       │       │   ├── auth-plan.md
│       │       │   ├── compression-plan.md
│       │       │   ├── cors-plan.md
│       │       │   ├── logging-plan.md
│       │       │   ├── metrics-plan.md
│       │       │   ├── mod-plan.md
│       │       │   ├── pipeline-plan.md
│       │       │   ├── rate_limit-plan.md
│       │       │   └── security-plan.md
│       │       ├── request
│       │       │   ├── body-plan.md
│       │       │   ├── headers-plan.md
│       │       │   ├── mod-plan.md
│       │       │   ├── multipart-plan.md
│       │       │   └── request-plan.md
│       │       ├── response
│       │       │   ├── body-plan.md
│       │       │   ├── compression-plan.md
│       │       │   ├── mod-plan.md
│       │       │   ├── response-plan.md
│       │       │   └── streaming-plan.md
│       │       ├── routing
│       │       │   ├── matcher-plan.md
│       │       │   ├── mod-plan.md
│       │       │   ├── params-plan.md
│       │       │   ├── router-plan.md
│       │       │   └── tree-plan.md
│       │       ├── server
│       │       │   ├── connection-plan.md
│       │       │   ├── http1-plan.md
│       │       │   ├── http2-plan.md
│       │       │   ├── http3-plan.md
│       │       │   ├── mod-plan.md
│       │       │   └── tls-plan.md
│       │       └── utils
│       │           ├── atomic-plan.md
│       │           ├── mod-plan.md
│       │           ├── pool-plan.md
│       │           └── simd-plan.md
│       ├── angelax-db
│       │   └── src
│       │       ├── lib-plan.md
│       │       └── orm
│       │           └── mod-plan.md
│       ├── angelax-json
│       │   └── src
│       │       ├── lib-plan.md
│       │       ├── parser-plan.md
│       │       ├── serializer-plan.md
│       │       ├── simd-plan.md
│       │       ├── streaming-plan.md
│       │       └── validation-plan.md
│       ├── angelax-macros
│       │   └── src
│       │       ├── codegen-plan.md
│       │       ├── handler-plan.md
│       │       ├── lib-plan.md
│       │       ├── middleware-plan.md
│       │       ├── model-plan.md
│       │       ├── route-plan.md
│       │       └── validation-plan.md
│       └── angelax-runtime
│           └── src
│               ├── executor-plan.md
│               ├── io-plan.md
│               ├── lib-plan.md
│               ├── reactor-plan.md
│               ├── task-plan.md
│               └── time-plan.md
├── README.md
├── benchmarks
│   ├── analysis
│   ├── competitors
│   ├── results
│   └── scenarios
├── bindings
│   ├── csharp
│   │   ├── Angelex.Client.csproj
│   │   ├── Client.cs
│   │   ├── README.md
│   │   └── Types.cs
│   ├── go
│   │   ├── README.md
│   │   ├── client.go
│   │   ├── codegen.go
│   │   ├── go.mod
│   │   └── types.go
│   ├── java
│   │   ├── README.md
│   │   ├── pom.xml
│   │   └── src
│   │       ├── main
│   │       │   └── java
│   │       │       └── angelex
│   │       └── test
│   │           └── java
│   ├── javascript
│   │   ├── README.md
│   │   ├── package.json
│   │   ├── src
│   │   │   ├── client.ts
│   │   │   ├── codegen.ts
│   │   │   ├── index.ts
│   │   │   └── types.ts
│   │   └── tests
│   └── python
│       ├── README.md
│       ├── pyproject.toml
│       ├── src
│       │   └── angelex_client
│       │       ├── __init__.py
│       │       ├── client.py
│       │       ├── codegen.py
│       │       └── types.py
│       └── tests
├── crates
│   ├── angelax-allocator
│   │   ├── Cargo.toml
│   │   ├── benches
│   │   ├── src
│   │   │   └── lib.rs
│   │   └── tests
│   ├── angelax-auth
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── jwt
│   │   │   │   ├── algorithms.rs
│   │   │   │   ├── claims.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── token.rs
│   │   │   │   └── validation.rs
│   │   │   ├── lib.rs
│   │   │   ├── oauth
│   │   │   │   ├── client.rs
│   │   │   │   ├── flows.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── providers.rs
│   │   │   │   └── server.rs
│   │   │   ├── password
│   │   │   │   ├── argon2.rs
│   │   │   │   ├── bcrypt.rs
│   │   │   │   ├── hash.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── scrypt.rs
│   │   │   ├── rbac
│   │   │   │   ├── evaluation.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── permission.rs
│   │   │   │   ├── policy.rs
│   │   │   │   └── role.rs
│   │   │   ├── security
│   │   │   │   ├── audit.rs
│   │   │   │   ├── csrf.rs
│   │   │   │   ├── headers.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── xss.rs
│   │   │   └── session
│   │   │       ├── cookie.rs
│   │   │       ├── database.rs
│   │   │       ├── memory.rs
│   │   │       ├── mod.rs
│   │   │       └── redis.rs
│   │   └── tests
│   ├── angelax-cli
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── commands
│   │   │   │   ├── benchmark.rs
│   │   │   │   ├── build.rs
│   │   │   │   ├── deploy.rs
│   │   │   │   ├── dev.rs
│   │   │   │   ├── generate.rs
│   │   │   │   ├── migrate.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── new.rs
│   │   │   │   └── test.rs
│   │   │   ├── dev_server
│   │   │   │   ├── mod.rs
│   │   │   │   ├── proxy.rs
│   │   │   │   ├── reload.rs
│   │   │   │   ├── ssl.rs
│   │   │   │   └── watcher.rs
│   │   │   ├── generator
│   │   │   │   ├── clients.rs
│   │   │   │   ├── docs.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── models.rs
│   │   │   │   ├── routes.rs
│   │   │   │   └── tests.rs
│   │   │   ├── main.rs
│   │   │   ├── templates
│   │   │   │   ├── api.rs
│   │   │   │   ├── enterprise.rs
│   │   │   │   ├── full_stack.rs
│   │   │   │   ├── microservice.rs
│   │   │   │   ├── minimal.rs
│   │   │   │   └── mod.rs
│   │   │   └── utils
│   │   │       ├── config.rs
│   │   │       ├── fs.rs
│   │   │       ├── git.rs
│   │   │       ├── mod.rs
│   │   │       └── spinner.rs
│   │   └── tests
│   ├── angelax-common
│   │   ├── Cargo.toml
│   │   ├── benches
│   │   ├── src
│   │   │   ├── constants.rs
│   │   │   ├── error.rs
│   │   │   ├── lib.rs
│   │   │   ├── traits.rs
│   │   │   └── types.rs
│   │   └── tests
│   ├── angelax-config
│   │   ├── Cargo.toml
│   │   ├── examples
│   │   ├── src
│   │   │   ├── lib.rs
│   │   │   ├── loader.rs
│   │   │   ├── macros.rs
│   │   │   ├── source.rs
│   │   │   └── validation.rs
│   │   └── tests
│   ├── angelax-core
│   │   ├── Cargo.toml
│   │   ├── benches
│   │   ├── examples
│   │   ├── src
│   │   │   ├── config
│   │   │   │   ├── app_config.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── settings.rs
│   │   │   ├── error
│   │   │   │   ├── error.rs
│   │   │   │   ├── handler.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── recovery.rs
│   │   │   ├── lib.rs
│   │   │   ├── middleware
│   │   │   │   ├── auth.rs
│   │   │   │   ├── compression.rs
│   │   │   │   ├── cors.rs
│   │   │   │   ├── logging.rs
│   │   │   │   ├── metrics.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── pipeline.rs
│   │   │   │   ├── rate_limit.rs
│   │   │   │   └── security.rs
│   │   │   ├── request
│   │   │   │   ├── body.rs
│   │   │   │   ├── headers.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── multipart.rs
│   │   │   │   └── request.rs
│   │   │   ├── response
│   │   │   │   ├── body.rs
│   │   │   │   ├── compression.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── response.rs
│   │   │   │   └── streaming.rs
│   │   │   ├── routing
│   │   │   │   ├── matcher.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── params.rs
│   │   │   │   ├── router.rs
│   │   │   │   └── tree.rs
│   │   │   ├── server
│   │   │   │   ├── connection.rs
│   │   │   │   ├── http1.rs
│   │   │   │   ├── http2.rs
│   │   │   │   ├── http3.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── tls.rs
│   │   │   └── utils
│   │   │       ├── atomic.rs
│   │   │       ├── mod.rs
│   │   │       ├── pool.rs
│   │   │       └── simd.rs
│   │   └── tests
│   ├── angelax-db
│   │   ├── Cargo.toml
│   │   ├── migrations
│   │   ├── src
│   │   │   ├── cache
│   │   │   │   ├── distributed.rs
│   │   │   │   ├── memcached.rs
│   │   │   │   ├── memory.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── redis.rs
│   │   │   ├── drivers
│   │   │   │   ├── elasticsearch.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── mongodb.rs
│   │   │   │   ├── mysql.rs
│   │   │   │   ├── postgres.rs
│   │   │   │   ├── redis.rs
│   │   │   │   └── sqlite.rs
│   │   │   ├── lib.rs
│   │   │   ├── orm
│   │   │   │   ├── migrations.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── model.rs
│   │   │   │   ├── query.rs
│   │   │   │   ├── relations.rs
│   │   │   │   └── schema.rs
│   │   │   ├── pool
│   │   │   │   ├── connection.rs
│   │   │   │   ├── health_check.rs
│   │   │   │   ├── load_balancer.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── pool.rs
│   │   │   └── transaction
│   │   │       ├── distributed.rs
│   │   │       ├── isolation.rs
│   │   │       ├── mod.rs
│   │   │       └── transaction.rs
│   │   └── tests
│   ├── angelax-deployment
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── cloud
│   │   │   │   ├── aws.rs
│   │   │   │   ├── azure.rs
│   │   │   │   ├── digital_ocean.rs
│   │   │   │   ├── gcp.rs
│   │   │   │   └── mod.rs
│   │   │   ├── docker
│   │   │   │   ├── builder.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── optimization.rs
│   │   │   │   └── security.rs
│   │   │   ├── edge
│   │   │   │   ├── cdn.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── regions.rs
│   │   │   │   └── workers.rs
│   │   │   ├── kubernetes
│   │   │   │   ├── ingress.rs
│   │   │   │   ├── manifests.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── operator.rs
│   │   │   │   └── scaling.rs
│   │   │   ├── lib.rs
│   │   │   └── serverless
│   │   │       ├── cloud_functions.rs
│   │   │       ├── cloud_run.rs
│   │   │       ├── lambda.rs
│   │   │       └── mod.rs
│   │   └── tests
│   ├── angelax-graphql
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── execution.rs
│   │   │   ├── introspection.rs
│   │   │   ├── lib.rs
│   │   │   ├── resolver.rs
│   │   │   ├── schema.rs
│   │   │   ├── subscription.rs
│   │   │   └── validation.rs
│   │   └── tests
│   ├── angelax-grpc
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── client.rs
│   │   │   ├── codec.rs
│   │   │   ├── lib.rs
│   │   │   ├── reflection.rs
│   │   │   ├── server.rs
│   │   │   └── streaming.rs
│   │   └── tests
│   ├── angelax-json
│   │   ├── Cargo.toml
│   │   ├── benches
│   │   ├── src
│   │   │   ├── lib.rs
│   │   │   ├── parser.rs
│   │   │   ├── serializer.rs
│   │   │   ├── simd.rs
│   │   │   ├── streaming.rs
│   │   │   └── validation.rs
│   │   └── tests
│   ├── angelax-macros
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── codegen.rs
│   │   │   ├── handler.rs
│   │   │   ├── lib.rs
│   │   │   ├── middleware.rs
│   │   │   ├── model.rs
│   │   │   ├── route.rs
│   │   │   └── validation.rs
│   │   └── tests
│   ├── angelax-monitoring
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── alerting
│   │   │   │   ├── channels.rs
│   │   │   │   ├── escalation.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── rules.rs
│   │   │   ├── health
│   │   │   │   ├── checks.rs
│   │   │   │   ├── dependencies.rs
│   │   │   │   ├── endpoints.rs
│   │   │   │   └── mod.rs
│   │   │   ├── lib.rs
│   │   │   ├── logging
│   │   │   │   ├── appenders.rs
│   │   │   │   ├── formatters.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── structured.rs
│   │   │   ├── metrics
│   │   │   │   ├── collector.rs
│   │   │   │   ├── custom.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── prometheus.rs
│   │   │   │   └── registry.rs
│   │   │   └── tracing
│   │   │       ├── jaeger.rs
│   │   │       ├── mod.rs
│   │   │       ├── span.rs
│   │   │       ├── trace.rs
│   │   │       └── zipkin.rs
│   │   └── tests
│   ├── angelax-plugin-api
│   │   ├── Cargo.toml
│   │   ├── examples
│   │   └── src
│   │       ├── error.rs
│   │       ├── lib.rs
│   │       ├── traits.rs
│   │       └── types.rs
│   ├── angelax-runtime
│   │   ├── Cargo.toml
│   │   ├── benches
│   │   ├── src
│   │   │   ├── executor.rs
│   │   │   ├── io.rs
│   │   │   ├── lib.rs
│   │   │   ├── reactor.rs
│   │   │   ├── task.rs
│   │   │   └── time.rs
│   │   └── tests
│   ├── angelax-testing
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── e2e
│   │   │   │   ├── api.rs
│   │   │   │   ├── browser.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── scenarios.rs
│   │   │   ├── integration
│   │   │   │   ├── client.rs
│   │   │   │   ├── database.rs
│   │   │   │   ├── fixtures.rs
│   │   │   │   └── mod.rs
│   │   │   ├── lib.rs
│   │   │   ├── load
│   │   │   │   ├── generator.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── reporter.rs
│   │   │   │   └── runner.rs
│   │   │   ├── property
│   │   │   │   ├── generators.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── shrinking.rs
│   │   │   │   └── strategies.rs
│   │   │   └── unit
│   │   │       ├── assertions.rs
│   │   │       ├── framework.rs
│   │   │       ├── mocks.rs
│   │   │       └── mod.rs
│   │   └── tests
│   └── angelax-websocket
│       ├── Cargo.toml
│       ├── src
│       │   ├── broadcast.rs
│       │   ├── compression.rs
│       │   ├── connection.rs
│       │   ├── frame.rs
│       │   ├── lib.rs
│       │   ├── message.rs
│       │   ├── protocol.rs
│       │   └── room.rs
│       └── tests
├── docker
│   ├── Dockerfile
│   ├── Dockerfile.alpine
│   ├── Dockerfile.distroless
│   ├── docker-compose.dev.yml
│   ├── docker-compose.prod.yml
│   └── docker-compose.yml
├── docs
│   ├── api-reference
│   │   ├── allocator
│   │   ├── auth
│   │   ├── common
│   │   ├── config
│   │   ├── core
│   │   ├── database
│   │   ├── deployment
│   │   ├── middleware
│   │   ├── plugin-api
│   │   ├── routing
│   │   ├── runtime
│   │   └── testing
│   ├── cookbook
│   │   ├── authentication.md
│   │   ├── caching.md
│   │   ├── database-patterns.md
│   │   ├── deployment-patterns.md
│   │   └── monitoring.md
│   ├── examples
│   │   ├── enterprise-app
│   │   ├── graphql-api
│   │   ├── hello-world
│   │   ├── microservices
│   │   ├── rest-api
│   │   └── websocket-chat
│   ├── getting-started
│   │   ├── concepts.md
│   │   ├── first-app.md
│   │   ├── installation.md
│   │   └── quick-start.md
│   └── guides
│       ├── architecture.md
│       ├── migration.md
│       ├── performance.md
│       ├── scaling.md
│       └── security.md
├── examples
│   ├── enterprise-app
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   ├── docs
│   │   ├── migrations
│   │   ├── src
│   │   │   ├── config
│   │   │   ├── main.rs
│   │   │   ├── middleware
│   │   │   ├── modules
│   │   │   └── utils
│   │   └── tests
│   ├── graphql-api
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src
│   │       ├── main.rs
│   │       ├── models
│   │       ├── resolvers
│   │       └── schema
│   ├── hello-world
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   └── src
│   │       └── main.rs
│   ├── microservices
│   │   ├── README.md
│   │   ├── api-gateway
│   │   ├── auth-service
│   │   ├── docker-compose.yml
│   │   └── user-service
│   ├── performance-showcase
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   ├── benchmarks
│   │   └── src
│   │       └── main.rs
│   ├── rest-api
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   ├── migrations
│   │   └── src
│   │       ├── handlers
│   │       ├── main.rs
│   │       ├── models
│   │       └── routes
│   └── websocket-chat
│       ├── Cargo.toml
│       ├── README.md
│       ├── src
│       │   ├── client
│       │   ├── handlers
│       │   ├── main.rs
│       │   └── rooms
│       └── static
├── ide-plugins
│   ├── emacs
│   │   ├── README.md
│   │   ├── angelex-lsp.el
│   │   └── angelex-mode.el
│   ├── intellij
│   │   ├── README.md
│   │   ├── build.gradle
│   │   └── src
│   │       └── main
│   │           ├── java
│   │           └── resources
│   ├── vim
│   │   ├── README.md
│   │   ├── ftdetect
│   │   │   └── angelex.vim
│   │   ├── plugin
│   │   │   └── angelex.vim
│   │   └── syntax
│   │       └── angelex.vim
│   └── vscode
│       ├── README.md
│       ├── package.json
│       ├── src
│       │   ├── commands.ts
│       │   ├── debugger.ts
│       │   ├── extension.ts
│       │   ├── language-server.ts
│       │   └── snippets.ts
│       ├── syntaxes
│       └── themes
├── kubernetes
│   ├── configmap.yaml
│   ├── deployment.yaml
│   ├── hpa.yaml
│   ├── ingress.yaml
│   ├── monitoring.yaml
│   ├── namespace.yaml
│   ├── secret.yaml
│   └── service.yaml
├── monitoring
│   ├── alertmanager
│   │   └── alertmanager.yml
│   ├── grafana
│   │   ├── dashboards
│   │   └── provisioning
│   ├── jaeger
│   │   └── jaeger.yml
│   └── prometheus
│       ├── prometheus.yml
│       └── rules
├── scripts
│   ├── benchmark.sh
│   ├── build.sh
│   ├── install.sh
│   ├── release.sh
│   ├── setup-dev.sh
│   └── test.sh
├── terraform
│   ├── environments
│   │   ├── dev
│   │   ├── prod
│   │   └── staging
│   ├── main.tf
│   ├── modules
│   │   ├── aws
│   │   ├── azure
│   │   └── gcp
│   ├── outputs.tf
│   └── variables.tf
├── tools
│   ├── benchmarks
│   │   ├── Cargo.toml
│   │   ├── results
│   │   └── src
│   │       ├── analysis
│   │       ├── competitors
│   │       ├── main.rs
│   │       ├── reporting
│   │       └── scenarios
│   ├── code-generator
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── generators
│   │   │   ├── main.rs
│   │   │   ├── parsers
│   │   │   └── templates
│   │   └── templates
│   ├── load-tester
│   │   ├── Cargo.toml
│   │   ├── results
│   │   ├── scenarios
│   │   └── src
│   │       ├── main.rs
│   │       ├── metrics
│   │       └── scenarios
│   ├── profiler
│   │   ├── Cargo.toml
│   │   ├── profiles
│   │   └── src
│   │       ├── cpu.rs
│   │       ├── io.rs
│   │       ├── main.rs
│   │       ├── memory.rs
│   │       └── visualization.rs
│   └── security-scanner
│       ├── Cargo.toml
│       ├── src
│       │   ├── analyzers
│       │   ├── main.rs
│       │   ├── reports
│       │   └── rules
│       └── vulnerability-db
└── website
    ├── index.html
    ├── package.json
    ├── public
    │   └── assets
    ├── src
    │   ├── App.tsx
    │   ├── assets
    │   │   ├── fonts
    │   │   └── images
    │   │       └── logo.svg
    │   ├── components
    │   │   ├── common
    │   │   │   └── Button
    │   │   │       ├── Button.module.css
    │   │   │       └── Button.tsx
    │   │   └── layout
    │   │       └── Navbar
    │   │           └── Navbar.tsx
    │   ├── features
    │   │   ├── auth
    │   │   │   ├── components
    │   │   │   │   └── LoginForm.tsx
    │   │   │   ├── hooks
    │   │   │   │   └── useAuth.ts
    │   │   │   ├── index.ts
    │   │   │   ├── services
    │   │   │   │   └── authService.ts
    │   │   │   ├── store
    │   │   │   │   └── authStore.ts
    │   │   │   └── types
    │   │   │       └── index.ts
    │   │   └── dashboard
    │   ├── hooks
    │   │   ├── useDebounce.ts
    │   │   └── useLocalStorage.ts
    │   ├── index.css
    │   ├── layouts
    │   │   └── AppLayout.tsx
    │   ├── lib
    │   │   ├── apiClient.ts
    │   │   ├── constants.ts
    │   │   └── utils.ts
    │   ├── main.tsx
    │   ├── pages
    │   │   ├── DashboardPage.tsx
    │   │   ├── HomePage.tsx
    │   │   ├── LoginPage.tsx
    │   │   └── NotFoundPage.tsx
    │   ├── routes
    │   │   ├── index.tsx
    │   │   └── protectedRoute.tsx
    │   ├── services
    │   │   └── userService.ts
    │   ├── store
    │   │   └── index.ts
    │   ├── styles
    │   │   ├── global.css
    │   │   ├── theme.ts
    │   │   └── variables.scss
    │   ├── types
    │   │   └── index.ts
    │   └── vite-env.d.ts
    ├── tsconfig.json
    ├── tsconfig.node.json
    ├── vite.config.js
    └── vite.config.ts

341 directories, 671 files
```
