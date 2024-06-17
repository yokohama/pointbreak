/*
 * curl -s -H 'Content-Type: application/json' \
 *   -H 'Authorization: Bearer <JWT>' \
 *   http://localhost:3000/users/protected
 *
 * (疑問)
 * なぜ、`protected`や`authorize`メソッドを読んでいるのに、
 * メソッドに定義されている引数を渡さなくても大丈夫なのか？
 *
 * (解答)
 * axumの仕様。axumは関数のシグネチャから必要な引数を自動的に解析
 * HTTPリクエストから引数に対応するデータを抽出する
 * `protected`メソッドの場合、引数にClaim型が宣言されている。
 * Claim型がFromRequestトレイトを実装しているので、axumがよしなにやっている。
 *
 * (axumよしなの詳細フロー)
 * 1. `protected`メソッドを探す
 * 2. `protected`メソッドのシグネチャを確認 > 引数にClaimsがある
 * 3. Claims構造体には、`FromRequestRarts`トレイトの実装がある
 * 4. 自動的に、Claimsの`from_request_parts`を実行する
 * 5. その際、引数の`Parts`には、リクエストの内容が渡される(axumのよしな)
 */

use crate::services::authorization::{
    jwt::Claims,
    auth::AuthError,
};

pub async fn index(claims: Claims) -> Result<String, AuthError> {
    Ok(format!(
        "Welcome2 to the protected area :)\nYour data:\n{claims}",
    ))
}
