use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct LokiResponse {
    pub status: String,
    pub data: LokiData,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct LokiData {
    #[serde(rename = "resultType")]
    pub result_type: String,
    pub result: Vec<LokiStream>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct LokiStream {
    pub stream: HashMap<String, String>, // Метаданные потока (например, {"job": "my-job"})
    pub values: Vec<(String, String)>,
}

pub fn invoke_loki_get_api(loki_url: &str, query: &str, start: u64, end: u64) -> LokiResponse {
    let client = reqwest::blocking::Client::new();
    println!("query {}", query);
    let response = client
        .get(&format!("{}/loki/api/v1/query_range", loki_url))
        .header("X-Scope-OrgID", "1")
        .query(&[
            ("query", query),
            ("start", &start.to_string()),
            ("end", &end.to_string()),
            ("limit", "50"),
        ])
        .send()
        .unwrap();
    response.json().unwrap()
}

#[cfg(test)]
mod test {
    use crate::loki::{invoke_loki_get_api, LokiResponse};
    use mockito::Server;
    use serde_json::{json, Value};

    #[test]
    fn parse_test() {
        let original_json = get_test_json();
        let response: LokiResponse = serde_json::from_value(original_json.clone()).unwrap();
        let actual_json = serde_json::to_value(&response).unwrap();
        assert_eq!(original_json, actual_json);
    }

    #[test]
    fn test_query_loki() {
        let mut server = Server::new();
        let mock_response = json!({
            "status": "success",
            "data": {
                "resultType": "streams",
                "result": [
                    {
                        "stream": {"job": "my-job"},
                        "values": [["1631234567890000000", "log message"]]
                    }
                ]
            }
        });

        let _m = server
            .mock("GET", "/loki/api/v1/query_range")
            .match_query(mockito::Matcher::UrlEncoded(
                "query".into(),
                "{job=\"my-job\"}".into(),
            ))
            .match_query(mockito::Matcher::UrlEncoded(
                "start".into(),
                "1631234567".into(),
            ))
            .match_query(mockito::Matcher::UrlEncoded(
                "end".into(),
                "1631234687".into(),
            ))
            .match_query(mockito::Matcher::UrlEncoded("limit".into(), "50".into()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let loki_url = server.url();
        let query = "{job=\"my-job\"}";
        let start = 1631234567;
        let end = 1631234687;

        let result = invoke_loki_get_api(&loki_url, query, start, end);
        assert_eq!(result.status, "success");
        assert_eq!(result.data.result_type, "streams");
        assert_eq!(result.data.result.len(), 1);
        assert_eq!(result.data.result[0].stream["job"], "my-job");
        assert_eq!(result.data.result[0].values[0].0, "1631234567890000000");
        assert_eq!(result.data.result[0].values[0].1, "log message");
    }

    fn get_test_json() -> Value {
        json!({
               "status": "success",
                "data": {
                    "resultType": "streams",
                    "result": [
                          {
                            "stream": {
                              "app": "bb",
                              "container": "cheetah",
                              "detected_level": "ERROR",
                              "filename": "/var/log/pods/prod1/cheetah/1.log",
                              "job": "prod/bb",
                              "level": "ERROR",
                              "namespace": "prod",
                              "node_name": "blue-euzh0",
                              "pod": "bb",
                              "service_name": "bb",
                              "stream": "stdout"
                            },
                            "values": [
                              [
                                "1742448728898447002",
                                "2025-03-20T05:32:08.898179Z ERROR Error execute command: Event(BinaryField { object_id: GameObjectId { id: 1014, is_room_owner: true, member_id: 0 }, field_id: 3005, value: Buffer { buffer: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] } }) in room 2 from client 10 : GameObjectNotFound { object_id: GameObjectId { id: 1014, is_room_owner: true, member_id: 0 } }"
                              ]
                            ]
                          },
                          {
                            "stream": {
                              "app": "ff",
                              "container": "cheetah",
                              "detected_level": "INFO",
                              "filename": "/var/log/pods/prod_ff_7d5258fc-9462-43c8-863e-3b2ce6eb2adb/cheetah/1.log",
                              "job": "prod/ff",
                              "level": "INFO",
                              "namespace": "prod",
                              "node_name": "blue-a2td7",
                              "pod": "ff",
                              "service_name": "ff",
                              "stream": "stdout"
                            },
                            "values": [
                              [
                                "1742448729573298862",
                                "2025-03-20T05:32:09.573178Z  INFO Protocol: is disconnected Some(Command(ClientStopped))"
                              ]
                            ]
                          }
                     ],
                }
        })
    }
}
