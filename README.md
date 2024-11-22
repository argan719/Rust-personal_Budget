## Rust-personal_Budget
#### RUST, REST API(Rocket), SQLITE(DB), TAURI
<br>

### API 기반 개인 가계부 앱
개발 목표:<br>
금융 데이터를 추적하고 분석하는 데 도움을 주는 CLI/GUI 기반 앱 개발. <br><br>

활용 기술:<br>
Rocket: Rust 웹 프레임워크로 REST API 생성.<br>
SQLite 또는 PostgreSQL: 데이터 저장소.<br>
Tauri: GUI 지원. <br><br>

구현 단계:<br>
데이터 모델 설계: 거래(날짜, 금액, 카테고리) 데이터 구조 정의.<br>
REST API 생성: Rocket으로 CRUD 기능 개발.<br>
CLI/GUI 개발: Tauri로 간단한 데스크톱 앱 제작.<br>
배포: 바이너리 패키징 후 배포.<br>

<br>

목표:<br>
Rust로 REST API와 GUI/CLI 앱을 만들어 개인 가계부를 관리하는 프로그램 개발. 사용자는 수입과 지출 기록, 데이터 조회 및 분석 가능.<br>

<br>

#### 프로젝트 개요
1. REST API<br>
사용자가 수입/지출 데이터를 생성, 조회, 수정, 삭제(CRUD)할 수 있도록 지원.<br>
항목별 통계 기능 제공 (월별 지출 합계, 카테고리별 분석 등).<br>

2. Database<br>
SQLite/PostgreSQL 사용. 데이터를 효율적으로 저장하고 쿼리 지원.<br>

3. CLI/GUI<br>
CLI: 터미널에서 간단한 명령으로 데이터를 조작 가능.<br>
GUI: Tauri를 사용해 데스크톱 애플리케이션 제공.<br>
