## [0.1.10] - 2025-08-30

### 📚 Documentation

- Add unified CHANGELOG.md for dxc package ([`d5aceee`](https://github.com/efahnjoe/dxc/commit/d5aceee84162026931b70268be5e4ebf3af867e3))


### ⚙️ Miscellaneous Tasks

- Add CHANGELOG.md files for dxc packages ([`c3002f6`](https://github.com/efahnjoe/dxc/commit/c3002f69f6dec3a320917e6e7a3b2447e1a6e40c))

- Release v0.1.9 (#13) ([`9685d16`](https://github.com/efahnjoe/dxc/commit/9685d16a9ca50fbd63fe8751fe3ff4b250735042))

- Remove `release-plz` workflow ([`ddb4677`](https://github.com/efahnjoe/dxc/commit/ddb467728f389665effdbe8a280b1510f0dc8260))

- Add GitHub Actions workflow to publish crate on tag push ([`c8d9c8e`](https://github.com/efahnjoe/dxc/commit/c8d9c8e06aae0face230bef27889d5ea7e15e2cb))

- *(husky)* Remove error code from pre-commit hook ([`fc62449`](https://github.com/efahnjoe/dxc/commit/fc62449d074345668fa624715776bae4eca86c49))

- *(release)* Release v0.1.10 ([`0c265de`](https://github.com/efahnjoe/dxc/commit/0c265de5958e6890e52d361d3c86a1afbcf3ee7e))

## 1.0.0 (2026-02-27)


### Features

* **Button:** implement customizable button component ([9d14df9](https://github.com/efahnjoe/dxc/commit/9d14df98d039cc8533e41972ad19172c49cd5d92))
* **components/button:** introduce DxcButtonGroup component for button collections ([ef704a7](https://github.com/efahnjoe/dxc/commit/ef704a79ec4ca88ff71f890e04d23029e352f853))
* **components/image:** introduce DxcImage component with enhanced features ([5848fc2](https://github.com/efahnjoe/dxc/commit/5848fc254557fdafa2d78910869c125df56ec4c1))
* **components/tooltip:** scaffold Tooltip component structure ([2b99bc8](https://github.com/efahnjoe/dxc/commit/2b99bc86670977097f08671cd3d63e3fbeabdb87))
* **components:** enhance container and icon components ([f9af4fc](https://github.com/efahnjoe/dxc/commit/f9af4fc9dffaf8151b54d6284f06e439856c7f48))
* **constants:** introduce event code constants module ([391a7a8](https://github.com/efahnjoe/dxc/commit/391a7a82c67a7feaaf0edeaeaa2c94fe5e623885))
* **dxc-components/focus_trap:** implement focus trapping component for accessibility ([3587279](https://github.com/efahnjoe/dxc/commit/35872796e6eb129ef2c1cbb7b7f86c909a5dfeaa))
* **dxc-components/icon:** add id attribute support to Icon component ([a0419b5](https://github.com/efahnjoe/dxc/commit/a0419b57e022d36ebeeedc822f9a85a03522ea32))
* **dxc-components/icon:** add onClick event handling ([7d497b0](https://github.com/efahnjoe/dxc/commit/7d497b0fdd78a428ed82bc05719879e46b828b6c))
* **dxc-components/image_viewer:** add image viewer component with zoom and rotate support ([1f50889](https://github.com/efahnjoe/dxc/commit/1f50889915479f35047db40b681af7b53d1f6f75))
* **dxc-components/input:** enhance Input component with new features and improvements ([f459be0](https://github.com/efahnjoe/dxc/commit/f459be00efef1cd0b5f03df5ad02d914ad24643b))
* **dxc-components/teleprot:** add Teleport component ([de012c8](https://github.com/efahnjoe/dxc/commit/de012c8e6e426d79206d01cea7c38087c3390423))
* **dxc-components/transition:** add Transition component ([9fbf77f](https://github.com/efahnjoe/dxc/commit/9fbf77f9e9c67e136aed7b39caa367da5743cd6d))
* **dxc-components:** enable focus_trap and image_viewer, add teleport/transition, and re-export components ([48d7d8e](https://github.com/efahnjoe/dxc/commit/48d7d8e56a561f0a0b676920bc0d663d58f13fb1))
* **dxc-macros/props_default:** introduce PropsDefault macro for configurable defaults ([53d69a8](https://github.com/efahnjoe/dxc/commit/53d69a854f2b5ed52ce1d3f1b1f2a3ccd81b39a5))
* **dxc-macros/props:** enhance props macro with custom derive path support ([65cb249](https://github.com/efahnjoe/dxc/commit/65cb249fb6985f6e437bd2f8ed2139c87c4685d7))
* **dxc-macros/props:** enhance Props macro with field attributes and improved parsing ([5cea766](https://github.com/efahnjoe/dxc/commit/5cea7668224a06a5e2c00a2c9ad6add47d28fe42))
* **dxc-types/crossorigin:** add Crossorigin enum with trait implementations ([b3d872f](https://github.com/efahnjoe/dxc/commit/b3d872f4d61437a7b7eacab2207e175d0afb9bcd))
* **dxc-types/direction:** add Direction enum for layout orientation ([2f2458d](https://github.com/efahnjoe/dxc/commit/2f2458d500321d898d1e8c1d6ee972eb17b50e53))
* **dxc-types/direction:** add IntoAttributeValue impl for Direction with kebab-case serialization ([d945a1c](https://github.com/efahnjoe/dxc/commit/d945a1cb095fe41da8efcb9df6a388f8b0fcfdd7))
* **dxc-types/namespace:** introduce namespace module with Block enum ([ce63865](https://github.com/efahnjoe/dxc/commit/ce63865a2270eea934c2a4484f990fc312c4f9eb))
* **dxc-types/resize:** introduce Resize enum for DxcInput sizing control ([e37ec0c](https://github.com/efahnjoe/dxc/commit/e37ec0c7005f72036c513647b3135b1e4eb53c2e))
* **dxc-types/size:** introduce Size enum with serialization and Dioxus support ([2dd4984](https://github.com/efahnjoe/dxc/commit/2dd49843b1fa1879ae98083d9d81b503708f3374))
* **dxc-types:** add Fit and Loading enumerations ([2cfef8e](https://github.com/efahnjoe/dxc/commit/2cfef8e7fe1551de3d561a522e533292ec987a2b))
* **dxc:** introduce dxc-types module and standardize imports ([59eca14](https://github.com/efahnjoe/dxc/commit/59eca14d855ddaaec37f2e37ae5a4fc23d18252e))
* **examples/image:** enhance image component demo with dynamic content and progress indicators ([8fbd63c](https://github.com/efahnjoe/dxc/commit/8fbd63c01fdcfdb873f00c0c0456e8ad0286d67e))
* **examples:** add Button component showcase ([8217165](https://github.com/efahnjoe/dxc/commit/82171654166c2e120b0412faa1e886b62b46c95c))
* **examples:** add Input component usage examples ([8585b8c](https://github.com/efahnjoe/dxc/commit/8585b8c145a9f6764743608428c2c13a3e8fa3d1))
* implement core layout components and theme system ([0046935](https://github.com/efahnjoe/dxc/commit/00469359939572c8c68aab5fe15581ec7742663c))
* **macros:** introduce macro library for CSS classes and props ([f4d6f16](https://github.com/efahnjoe/dxc/commit/f4d6f16f90a7a80dcadf46724cdd197e30bcd531))
* **macros:** refactor classes and props macros with procedural implementation ([3bc8da4](https://github.com/efahnjoe/dxc/commit/3bc8da42e6451da3f0aac1524bbacb5d10e8bd1a))
* **macros:** support default values in props macro ([5307927](https://github.com/efahnjoe/dxc/commit/53079273a8ffb7c05e08a6e0e5c2da4cca99e580))
* Refactor project into single-crate architecture ([#16](https://github.com/efahnjoe/dxc/issues/16)) ([7a24e72](https://github.com/efahnjoe/dxc/commit/7a24e72f5381a4bbb72f13997493c6d962def2ad))
* **types/block:** add Textarea variant to Block enum ([c38cbea](https://github.com/efahnjoe/dxc/commit/c38cbea82ed922f4a9139aba26b9f77fae62ee61))
* **types/namespace:** introduce new block types ([06049fc](https://github.com/efahnjoe/dxc/commit/06049fc4e31988116cb29875423bde8701b1d8c8))
* **types/types:** introduce ButtonType enum for semantic button styling ([7c12232](https://github.com/efahnjoe/dxc/commit/7c122325b6eef21716e6403dfc55a76ccf5f127e))
* **types/types:** introduce LinkType enum for semantic button styling ([fabd739](https://github.com/efahnjoe/dxc/commit/fabd739e23917bc3eacec06a2a926a2f0416c128))
* **workspace:** bump version to 0.1.6 and update README feature status ([f2b983f](https://github.com/efahnjoe/dxc/commit/f2b983fb4eede90de719b785188955ae214af3de))
* **workspace:** upgrade to Rust 2024 edition and update dependencies ([c9f0d2f](https://github.com/efahnjoe/dxc/commit/c9f0d2faf8c278c641eb8d010c7b60086e89909c))


### Bug Fixes

* **config:** correct cargo build target directory configuration ([062fcdc](https://github.com/efahnjoe/dxc/commit/062fcdc6a18ef7aaab59d7c348fd1c2817271df0))
* **dxc-components/input:** correct clear icon visibility logic and interaction ([0467b95](https://github.com/efahnjoe/dxc/commit/0467b9515edcca2d650621cbc34de37aeeaf1512))
* **dxc-components/input:** correct password input icon visibility ([71e3e23](https://github.com/efahnjoe/dxc/commit/71e3e23b03cbdc531ce4781aceb9b6e3e7f158e1))
* **dxc-components/input:** fix password icon toggle and input state handling ([63b0ec4](https://github.com/efahnjoe/dxc/commit/63b0ec42a0c6d8dee444f100e08ba142e5b686d7))
* **dxc-components/input:** resolve input value capture and password icon issues ([31d23af](https://github.com/efahnjoe/dxc/commit/31d23af0f08f21f61c36aadc2ca92410cda125a9))
* **example:** correct DxcLink component property name ([38f03b9](https://github.com/efahnjoe/dxc/commit/38f03b92bcb729af0e97ceeb8e30f0aa93fa39ce))
* **hooks:** correct namespace handling for None state in `UseNamespeace` ([ae6aa2c](https://github.com/efahnjoe/dxc/commit/ae6aa2c635aac0af52ffdb13ef6df5cbe108bf4a))
* **hooks:** correct the naming of elements and modifiers in the BEM naming convention ([932af45](https://github.com/efahnjoe/dxc/commit/932af45e21cd7d40a9032e82e27d3348710155e0))
* **hooks:** handle empty modifier in namespace `BEM` generation ([a1a18e2](https://github.com/efahnjoe/dxc/commit/a1a18e238914720c1e74ba157b160eeafa91ef2e))
* **hooks:** improve `is_` function logic in `use_namespace` ([fd5a0a6](https://github.com/efahnjoe/dxc/commit/fd5a0a67d8d3c5b0b1dd7716725a5c29bbf22560))
* **image-viewer:** fix key attribute ([90ae82b](https://github.com/efahnjoe/dxc/commit/90ae82b8e31bd0aaaea6642104d44276aafb2bfc))

## [0.1.8] - 2025-08-28

### 🚀 Features

- Implement core layout components and theme system ([`0046935`](https://github.com/efahnjoe/dxc/commit/00469359939572c8c68aab5fe15581ec7742663c))

- *(components)* Enhance container and icon components ([`f9af4fc`](https://github.com/efahnjoe/dxc/commit/f9af4fc9dffaf8151b54d6284f06e439856c7f48))

- *(components/image)* Introduce DxcImage component with enhanced features ([`5848fc2`](https://github.com/efahnjoe/dxc/commit/5848fc254557fdafa2d78910869c125df56ec4c1))

- *(constants)* Introduce event code constants module ([`391a7a8`](https://github.com/efahnjoe/dxc/commit/391a7a82c67a7feaaf0edeaeaa2c94fe5e623885))

- *(macros)* Introduce macro library for CSS classes and props ([`f4d6f16`](https://github.com/efahnjoe/dxc/commit/f4d6f16f90a7a80dcadf46724cdd197e30bcd531))

- *(macros)* Support default values in props macro ([`5307927`](https://github.com/efahnjoe/dxc/commit/53079273a8ffb7c05e08a6e0e5c2da4cca99e580))

- *(Button)* Implement customizable button component ([`9d14df9`](https://github.com/efahnjoe/dxc/commit/9d14df98d039cc8533e41972ad19172c49cd5d92))

- *(examples)* Add Button component showcase ([`8217165`](https://github.com/efahnjoe/dxc/commit/82171654166c2e120b0412faa1e886b62b46c95c))

- *(macros)* Refactor classes and props macros with procedural implementation ([`3bc8da4`](https://github.com/efahnjoe/dxc/commit/3bc8da42e6451da3f0aac1524bbacb5d10e8bd1a))

- *(dxc-components/icon)* Add onClick event handling ([`7d497b0`](https://github.com/efahnjoe/dxc/commit/7d497b0fdd78a428ed82bc05719879e46b828b6c))

- *(dxc-components/input)* Enhance Input component with new features and improvements ([`f459be0`](https://github.com/efahnjoe/dxc/commit/f459be00efef1cd0b5f03df5ad02d914ad24643b))

- *(dxc-components/icon)* Add id attribute support to Icon component ([`a0419b5`](https://github.com/efahnjoe/dxc/commit/a0419b57e022d36ebeeedc822f9a85a03522ea32))

- *(examples)* Add Input component usage examples ([`8585b8c`](https://github.com/efahnjoe/dxc/commit/8585b8c145a9f6764743608428c2c13a3e8fa3d1))

- *(dxc-types/direction)* Add Direction enum for layout orientation ([`2f2458d`](https://github.com/efahnjoe/dxc/commit/2f2458d500321d898d1e8c1d6ee972eb17b50e53))

- *(dxc)* Introduce dxc-types module and standardize imports ([`59eca14`](https://github.com/efahnjoe/dxc/commit/59eca14d855ddaaec37f2e37ae5a4fc23d18252e))

- *(dxc-macros/props)* Enhance Props macro with field attributes and improved parsing ([`5cea766`](https://github.com/efahnjoe/dxc/commit/5cea7668224a06a5e2c00a2c9ad6add47d28fe42))

- *(dxc-macros/props)* Enhance props macro with custom derive path support ([`65cb249`](https://github.com/efahnjoe/dxc/commit/65cb249fb6985f6e437bd2f8ed2139c87c4685d7))

- *(dxc-macros/props_default)* Introduce PropsDefault macro for configurable defaults ([`53d69a8`](https://github.com/efahnjoe/dxc/commit/53d69a854f2b5ed52ce1d3f1b1f2a3ccd81b39a5))

- *(dxc-types/direction)* Add IntoAttributeValue impl for Direction with kebab-case serialization ([`d945a1c`](https://github.com/efahnjoe/dxc/commit/d945a1cb095fe41da8efcb9df6a388f8b0fcfdd7))

- *(dxc-types/crossorigin)* Add Crossorigin enum with trait implementations ([`b3d872f`](https://github.com/efahnjoe/dxc/commit/b3d872f4d61437a7b7eacab2207e175d0afb9bcd))

- *(dxc-types)* Add Fit and Loading enumerations ([`2cfef8e`](https://github.com/efahnjoe/dxc/commit/2cfef8e7fe1551de3d561a522e533292ec987a2b))

- *(dxc-components/image_viewer)* Add image viewer component with zoom and rotate support ([`1f50889`](https://github.com/efahnjoe/dxc/commit/1f50889915479f35047db40b681af7b53d1f6f75))

- *(workspace)* Bump version to 0.1.6 and update README feature status ([`f2b983f`](https://github.com/efahnjoe/dxc/commit/f2b983fb4eede90de719b785188955ae214af3de))

- *(examples/image)* Enhance image component demo with dynamic content and progress indicators ([`8fbd63c`](https://github.com/efahnjoe/dxc/commit/8fbd63c01fdcfdb873f00c0c0456e8ad0286d67e))

- *(dxc-components/teleprot)* Add Teleport component ([`de012c8`](https://github.com/efahnjoe/dxc/commit/de012c8e6e426d79206d01cea7c38087c3390423))

- *(dxc-components/transition)* Add Transition component ([`9fbf77f`](https://github.com/efahnjoe/dxc/commit/9fbf77f9e9c67e136aed7b39caa367da5743cd6d))

- *(dxc-components/focus_trap)* Implement focus trapping component for accessibility ([`3587279`](https://github.com/efahnjoe/dxc/commit/35872796e6eb129ef2c1cbb7b7f86c909a5dfeaa))

- *(dxc-components)* Enable focus_trap and image_viewer, add teleport/transition, and re-export components ([`48d7d8e`](https://github.com/efahnjoe/dxc/commit/48d7d8e56a561f0a0b676920bc0d663d58f13fb1))

- *(dxc-types/size)* Introduce Size enum with serialization and Dioxus support ([`2dd4984`](https://github.com/efahnjoe/dxc/commit/2dd49843b1fa1879ae98083d9d81b503708f3374))

- *(dxc-types/namespace)* Introduce namespace module with Block enum ([`ce63865`](https://github.com/efahnjoe/dxc/commit/ce63865a2270eea934c2a4484f990fc312c4f9eb))

- *(dxc-types/resize)* Introduce Resize enum for DxcInput sizing control ([`e37ec0c`](https://github.com/efahnjoe/dxc/commit/e37ec0c7005f72036c513647b3135b1e4eb53c2e))

- *(types/block)* Add Textarea variant to Block enum ([`c38cbea`](https://github.com/efahnjoe/dxc/commit/c38cbea82ed922f4a9139aba26b9f77fae62ee61))

- *(types/types)* Introduce ButtonType enum for semantic button styling ([`7c12232`](https://github.com/efahnjoe/dxc/commit/7c122325b6eef21716e6403dfc55a76ccf5f127e))

- *(components/button)* Introduce DxcButtonGroup component for button collections ([`ef704a7`](https://github.com/efahnjoe/dxc/commit/ef704a79ec4ca88ff71f890e04d23029e352f853))

- *(types/types)* Introduce LinkType enum for semantic button styling ([`fabd739`](https://github.com/efahnjoe/dxc/commit/fabd739e23917bc3eacec06a2a926a2f0416c128))

- *(types/namespace)* Introduce new block types ([`06049fc`](https://github.com/efahnjoe/dxc/commit/06049fc4e31988116cb29875423bde8701b1d8c8))

- *(components/tooltip)* Scaffold Tooltip component structure ([`2b99bc8`](https://github.com/efahnjoe/dxc/commit/2b99bc86670977097f08671cd3d63e3fbeabdb87))


### 🐛 Bug Fixes

- *(hooks)* Correct the naming of elements and modifiers in the BEM naming convention ([`932af45`](https://github.com/efahnjoe/dxc/commit/932af45e21cd7d40a9032e82e27d3348710155e0))

- Add missed Cargo.toml from previous commit ([`0f7af95`](https://github.com/efahnjoe/dxc/commit/0f7af95eaedb1ec4fa16b0b61a0d1c318eb0e87e))

- *(hooks)* Improve `is_` function logic in `use_namespace` ([`fd5a0a6`](https://github.com/efahnjoe/dxc/commit/fd5a0a67d8d3c5b0b1dd7716725a5c29bbf22560))

- *(hooks)* Correct namespace handling for None state in `UseNamespeace` ([`ae6aa2c`](https://github.com/efahnjoe/dxc/commit/ae6aa2c635aac0af52ffdb13ef6df5cbe108bf4a))

- *(hooks)* Handle empty modifier in namespace `BEM` generation ([`a1a18e2`](https://github.com/efahnjoe/dxc/commit/a1a18e238914720c1e74ba157b160eeafa91ef2e))

- *(example)* Correct DxcLink component property name ([`38f03b9`](https://github.com/efahnjoe/dxc/commit/38f03b92bcb729af0e97ceeb8e30f0aa93fa39ce))

- *(dxc-components/input)* Correct password input icon visibility ([`71e3e23`](https://github.com/efahnjoe/dxc/commit/71e3e23b03cbdc531ce4781aceb9b6e3e7f158e1))

- *(dxc-components/input)* Resolve input value capture and password icon issues ([`31d23af`](https://github.com/efahnjoe/dxc/commit/31d23af0f08f21f61c36aadc2ca92410cda125a9))

- *(dxc-components/input)* Fix password icon toggle and input state handling ([`63b0ec4`](https://github.com/efahnjoe/dxc/commit/63b0ec42a0c6d8dee444f100e08ba142e5b686d7))

- *(dxc-components/input)* Correct clear icon visibility logic and interaction ([`0467b95`](https://github.com/efahnjoe/dxc/commit/0467b9515edcca2d650621cbc34de37aeeaf1512))


### 💼 Other

- Remove unused web-sys dependency ([`28c447e`](https://github.com/efahnjoe/dxc/commit/28c447e0a401c10cfec62ae14c2b8c484a914fba))

- *(dxc-types)* Add strum (with derive) and diooxus dependencies ([`c64c0b5`](https://github.com/efahnjoe/dxc/commit/c64c0b568b313fc2a3da255f182d24d55f0e3c37))

- *(workspace)* Bump version to 0.1.7 and update README feature tracking ([`0cfa34e`](https://github.com/efahnjoe/dxc/commit/0cfa34ee0b6dbbbfb5de5043c5225af9fe15c706))

- Introduce Makefile for streamlined project workflows ([`644ebfa`](https://github.com/efahnjoe/dxc/commit/644ebfa6145be1f99ce4b5d81e7183844858cf81))

- *(dxc-hooks)* Add dxc-types dependency ([`0a6cb9e`](https://github.com/efahnjoe/dxc/commit/0a6cb9e2c743f7445c3745d2d79aa7db3f669955))

- *(makefile)* Enhance versioning, formatting, and release workflow ([`7ccfa86`](https://github.com/efahnjoe/dxc/commit/7ccfa8690eea1b2391e173c45a14e61fdcb8b744))

- Bump dxc project version to v0.1.8 ([`f067b50`](https://github.com/efahnjoe/dxc/commit/f067b5083bd6547dd501491a91edd87870118c57))

- *(release)* Add release.toml and configure automated changelog and tagging ([`fdd70a4`](https://github.com/efahnjoe/dxc/commit/fdd70a40ffc0a20fee4fb8aca03cd22bb35fe7b9))

- Enforce uniform Rust version in workspace ([`3155299`](https://github.com/efahnjoe/dxc/commit/3155299d732158121ccb8f22fa7714eeffff36e5))

- *(cargo)* Standardize package descriptions and workspace configuration ([`6e9be75`](https://github.com/efahnjoe/dxc/commit/6e9be75fc7452d4f8787f542e5d133f25d9a0bd1))

- *(cargo)* Use version="0.1.8" for local dxc dependencies ([`555cdd4`](https://github.com/efahnjoe/dxc/commit/555cdd41bddbe2feeab5fd73d7e01443862233f3))


### 🚜 Refactor

- *(hooks)* Improve `is_` method in `use_namespace` hook ([`70b37d9`](https://github.com/efahnjoe/dxc/commit/70b37d9043acb6c3602e105bd212c1a409514616))

- *(components)* Modernize Link component with macro integration ([`cf1db63`](https://github.com/efahnjoe/dxc/commit/cf1db637d9c45993df3988d8965d78d95af195a2))

- *(icon)* Modernize Icon component props and styling ([`dc2b4b4`](https://github.com/efahnjoe/dxc/commit/dc2b4b4e1ca0546c1178e10a433e532117395a21))

- *(components)* Reorganize navigation components ([`a0d4396`](https://github.com/efahnjoe/dxc/commit/a0d43969645f3d55bf41927136ed0589a4b9d3e9))

- *(dxc-utils)* Remove unused `props` and `uuid` modules ([`35ea2a1`](https://github.com/efahnjoe/dxc/commit/35ea2a10c18eaf7c0b988a9d3247a31a58705965))

- *(dxc-components/icon)* Restructure icon component and improve styling support ([`0432ebb`](https://github.com/efahnjoe/dxc/commit/0432ebbec5d57f254f1c8885e9823ad836ffa105))

- *(dxc-macros)* Remove unused Punctuated type from PropsMacroInput ([`6b41f7c`](https://github.com/efahnjoe/dxc/commit/6b41f7c64dc4dbb12e9a9f4477948bf073994d84))

- *(dxc-macros/props)* Clean up impl_props function ([`5199105`](https://github.com/efahnjoe/dxc/commit/5199105ffdd990d0a792229363b4d68c3c219521))

- *(examples/image)* Revamp image example with preview functionality ([`c462ff0`](https://github.com/efahnjoe/dxc/commit/c462ff0f68db3b144d7bed96db8d221e4d47cf46))

- *(dxc-components/image)* Completely redesign image component with enhanced features ([`e572343`](https://github.com/efahnjoe/dxc/commit/e5723435fa5ba9b2eb4ceefb2c82c80f6825589c))

- *(dxc-components/button)* Restructure Button component with typed properties ([`cb019ab`](https://github.com/efahnjoe/dxc/commit/cb019abb9f807d78fde29412f3079fcf05b8015f))

- *(dxc-types/namespace)* Enhance namespace module with expanded Block variants ([`3584d7e`](https://github.com/efahnjoe/dxc/commit/3584d7e293d0dd03697edc63823f154cede684ba))

- *(dxc-hooks/use_namespace)* Modernize UseNamespace with owned string types ([`8e3cb17`](https://github.com/efahnjoe/dxc/commit/8e3cb1744d848ad217798f7ecf369509188291c3))

- *(dxc-components/aside)* Introduce dedicated Aside component with typed properties ([`a18338e`](https://github.com/efahnjoe/dxc/commit/a18338eaa9fae91f6b821112ce9a048d798887a3))

- *(dxc-components/footer)* Ntroduce dedicated `Footer` component with typed properties ([`a2302e2`](https://github.com/efahnjoe/dxc/commit/a2302e2c6f1bcf7c61da344863405e1578b3590d))

- *(dxc-components/main)* Ntroduce dedicated `main` component with typed properties ([`6255af6`](https://github.com/efahnjoe/dxc/commit/6255af6a62be8b65aa889558e91f4653110056be))

- *(dxc-components/header)* Ntroduce dedicated `header` component with typed properties ([`d28252b`](https://github.com/efahnjoe/dxc/commit/d28252be9beb46718bf6cf21391899ba01100b1a))

- *(dxc-components/container)* Introduce dedicated `container` component with typed properties ([`43c7f08`](https://github.com/efahnjoe/dxc/commit/43c7f08b73b2ea0a27694fa44c8cd17ef9f63001))

- *(dxc-components/icon)* Extract common properties and optimize implementation ([`d1f0609`](https://github.com/efahnjoe/dxc/commit/d1f06095a7a16b52f574945db5115a8ffb469aeb))

- *(components/button)* Modernize implementation and loading state management ([`2d5f3db`](https://github.com/efahnjoe/dxc/commit/2d5f3dbc31ab7d0563d682338e5140e28c832f38))

- *(components/input)* Modernize structure and type usage ([`e76e400`](https://github.com/efahnjoe/dxc/commit/e76e400014238be0f262d0d4dcb1072481019b18))

- *(components/image_viewer)* Clean up code and improve type safety ([`e06fe39`](https://github.com/efahnjoe/dxc/commit/e06fe39192e2b189a0e02398c9f4e01af4400468))

- *(components/link)* Modernize props management and reactivity ([`73b7e73`](https://github.com/efahnjoe/dxc/commit/73b7e7387e357eb9cfa850b2339ef9408adc2399))

- *(components/image)* Standardize namespace usage with typed identifiers ([`354a66c`](https://github.com/efahnjoe/dxc/commit/354a66c0a5c14e58d1cc2c102adf65184d880f0f))

- *(types)* Reorganize module structure and extract component properties ([`3ce6be0`](https://github.com/efahnjoe/dxc/commit/3ce6be0cbcf0aa96f0290eb5a688136c7dbf79ca))

- *(types)* Reorganize type definitions and streamline module structure ([`77b943a`](https://github.com/efahnjoe/dxc/commit/77b943a982514e9393fbc00336771136e4233729))

- *(container)* Simplify DxcContainer with use_signal and add style prop ([`23d851b`](https://github.com/efahnjoe/dxc/commit/23d851b74e4c3ce85f8c056ae1971a44ad8a0015))

- *(dxc)* Update type export paths in prelude ([`f68917a`](https://github.com/efahnjoe/dxc/commit/f68917ad49aced562a4c5157041c597047a34cac))

- *(example/link)* Integrate into main project and improve implementation ([`fb26ca8`](https://github.com/efahnjoe/dxc/commit/fb26ca8251c4885561db407c1e1498d4b348816d))

- *(examples)* Use enum types for button, container, and image properties ([`b770a3d`](https://github.com/efahnjoe/dxc/commit/b770a3d3e7bea343f09520f35e67b12fbef9d9db))

- *(components/menu)* Extract DxcMenuItem into standalone component ([`a5cc14d`](https://github.com/efahnjoe/dxc/commit/a5cc14da0490609001a61466a34f162a85d779d3))

- *(examples)* Rename and restructure example project ([`dd8df37`](https://github.com/efahnjoe/dxc/commit/dd8df3797e939f1b0cb436c68109e9be2bdbb756))

- *(workspace)* Remove unused constants package ([`f8619aa`](https://github.com/efahnjoe/dxc/commit/f8619aa4c118208ccca7a88e6b24e08c8fa48d70))


### 📚 Documentation

- Update README and add MIT LICENSE ([`c9f9842`](https://github.com/efahnjoe/dxc/commit/c9f984293276117fd67a1c365b582af29eedd785))

- *(macros)* Enhance documentation for `classes` and `props` macros ([`2ab5bc0`](https://github.com/efahnjoe/dxc/commit/2ab5bc0b8346a19b10a17e66540609ce422b79e3))

- *(README)* Restructure documentation and update component listing ([`fc4b9ad`](https://github.com/efahnjoe/dxc/commit/fc4b9ad24194451ab6859a7a83514601b81d6d73))

- *(README)* Reorganize component library documentation structure ([`3f0484e`](https://github.com/efahnjoe/dxc/commit/3f0484e107a16c58be44fd213486f2d33bf25325))

- *(dxc-components/icon)* Modernize documentation with Markdown links ([`5ef388b`](https://github.com/efahnjoe/dxc/commit/5ef388bde4e7d376f84eaa6f168d780902ba12d9))


### ⚙️ Miscellaneous Tasks

- *(vscode)* Refine editor config and enhance Rust/TOML formatting ([`2d56a35`](https://github.com/efahnjoe/dxc/commit/2d56a355a45c96725e8e6bb2acf72b2da62e33d7))

- Clean up unused files and code ([`513b64a`](https://github.com/efahnjoe/dxc/commit/513b64a69d72ca9121bf466072347a35d26eaf34))

- Add missing files from previous commit ([`269e266`](https://github.com/efahnjoe/dxc/commit/269e26601ee278310a93bf08acd0147bc4858baf))

- *(vscode)* Refine VSCode configuration settings ([`f21436c`](https://github.com/efahnjoe/dxc/commit/f21436c0429bcb18b794a14b268219b7a3f8e9a1))

- *(workspace)* Release version 0.1.5 ([`ca7f38b`](https://github.com/efahnjoe/dxc/commit/ca7f38b64c16635986a8921d1252beae18143c21))

- Rename fmt target to fmt-check to resolve conflicts ([`717db82`](https://github.com/efahnjoe/dxc/commit/717db820ac463375e75d759c2d7e5480cd27ce41))

- *(components/focus_trap)* Improve code formatting ([`e19ddd7`](https://github.com/efahnjoe/dxc/commit/e19ddd7772ea51ccac19f2b17d0e35e6fe85e698))

- *(release)* Streamline release process for pre-releases ([`58ac870`](https://github.com/efahnjoe/dxc/commit/58ac8707da1402df13b032a935effae46f731f9c))

- *(release)* Simplify release process and remove redundant options ([`96403e7`](https://github.com/efahnjoe/dxc/commit/96403e7d627b30c7d08e6127182d53432790d3fc))

- *(release)* Define execution steps in release process ([`9a4bf41`](https://github.com/efahnjoe/dxc/commit/9a4bf4100a8eb88a08711a65de6d2683e0d2f3f6))

- *(Makefile)* Add log target to generate CHANGELOG.md ([`ab1744e`](https://github.com/efahnjoe/dxc/commit/ab1744e39bc8339d675ef8e4d78560ba2d701485))

- Set up commit linting with Husky (Node.js) for Rust project ([`86c55b2`](https://github.com/efahnjoe/dxc/commit/86c55b25bec3db6ab1b909f3448c347ab3927507))

- Add release-plz continuous release workflow ([`a1f3688`](https://github.com/efahnjoe/dxc/commit/a1f368832819912b3f7689d36ee2802b98dc9e06))
