use crate::model::{ConfigApiDto, ConfigDto};
use crate::protocol_model;
use crate::protocol_model::{ConfigApiModel, ConfigModel};
use prost::{EncodeError, Message};

fn create_message(body: protocol_model::message::Body) -> protocol_model::Message {
    protocol_model::Message {
        body: Some(body),
    }
}

fn encode_container(messages: Vec<protocol_model::Message>) -> Result<Vec<u8>, EncodeError> {
    // Create a Container and add the message to it
    let container = protocol_model::Container {
        message: messages,
    };

    // Serialize the Container to a byte array
    let mut buf = Vec::new();
    buf.reserve(container.encoded_len());
    container.encode(&mut buf)?;
    Ok(buf)
}

pub fn create_document_response(documents: &Vec<ModelDocument>) -> Result<Vec<u8>, EncodeError> {
    let msg = protocol_model::DocumentResponse {
        documents: documents.iter().map(create_document).collect()
    };

    let message = create_message(
        protocol_model::message::Body::DocumentResponse(msg));
    encode_container(vec![message])
}

pub fn create_document_request(offset: u32, count: u32) -> Result<Vec<u8>, EncodeError> {
    let msg = protocol_model::DocumentRequest {
        offset,
        count,
    };

    let message = create_message(
        protocol_model::message::Body::DocumentRequest(msg));
    encode_container(vec![message])
}

//
// macro_rules! generate_message_create
// {
//     ($fn_name:ident, $type:ty) => {
//         fn $fn_name(model: &$type) -> protocol_model::ModelServerNotification {
//             protocol_model::ModelServerNotification {
//                 model_id: model.model_id,
//                 model_type: model.model_type,
//                 timestamp: model.timestamp,
//                 speed: model.speed,
//                 course: model.course,
//                 longitude: model.longitude,
//                 latitude: model.latitude,
//                 altitude: model.altitude,
//                 heading: model.heading,
//                 width: model.width,
//                 length: model.length,
//             }
//         }
//     }
// }
//
// macro_rules! generate_create_server_notification  {
//     ($fn_name:ident, $fn_create_message_name:ident, $type:ty) => {
//        pub(crate) fn $fn_name(msg_counter: &Arc<AtomicI32>, wall_time_delta: i64, models: &Vec<$type>, model_count: u32, model_id_mask: &Vec<u8>) -> Result<Vec<u8>, EncodeError> {
//             let mut messages = Vec::new();
//             for model in models {
//                 let notification = protocol_model::message::Body::ModelServerNotification($fn_create_message_name(model));
//                 messages.push(create_message(
//                     &msg_counter,
//                     wall_time_delta,
//                     notification));
//             }
//
//             messages.push(create_message(
//                 &msg_counter,
//                 wall_time_delta,
//                 protocol_model::message::Body::ModelViewServerNotification(protocol_model::ModelViewServerNotification {
//                     model_id_count: model_count as u32,
//                     model_id_mask: model_id_mask.to_owned()
//                 })));
//             encode_container(messages)
//         }
//     }
// }
//
// generate_message_create!(create_document_message, DocumentContactModel);
// generate_message_create!(create_telemetry_message, TelemetryModel);
// generate_message_create!(create_aircraft_message, AircraftModel);
//
// generate_create_server_notification!(create_radar_contact_server_notification, create_radar_contact_message, RadarContactModel);
// generate_create_server_notification!(create_telemetry_server_notification, create_telemetry_message, TelemetryModel);
// generate_create_server_notification!(create_flight_simulator_server_notification, create_aircraft_message, AircraftModel);

// pub(crate) fn create_ack_message(msg_id: i32, ack_msg_id: i32, wall_time_delta: i64) -> Result<Vec<u8>, EncodeError> {
//     let notification = protocol_model::message::Body::FlowControlClientNotification(
//         protocol_model::FlowControlClientNotification {
//             ack_msg_id,
//         });
//
//     let message = protocol_model::Message {
//         msg_id,
//         wall_time_delta,
//         body: Some(notification),
//     };
//     encode_container(vec![message])
// }