use crate::model::{ConfigApiDto, ConfigDto, VideoConfigDto, VideoDownloadConfigDto};
use crate::protocol_model::{ConfigApiModel, ConfigModel, VideoConfigModel, VideoDownloadConfigModel};

impl From<&ConfigApiDto> for ConfigApiModel {
    fn from(dto: &ConfigApiDto) -> Self {
        Self {
            host: dto.host.clone(),
            port: dto.port as u32,
            web_root: dto.web_root.clone(),
        }
    }
}

impl From<&VideoDownloadConfigDto> for VideoDownloadConfigModel {
    fn from(dto: &VideoDownloadConfigDto) -> Self {
        Self {
            headers: dto.headers.clone(),
            directory: dto.directory.to_string(),
            organize_into_directories: dto.organize_into_directories,
            episode_pattern: dto.episode_pattern.to_string(),
        }
    }
}

impl From<&VideoConfigDto> for VideoConfigModel {
    fn from(dto: &VideoConfigDto) -> Self {
        Self {
            extensions: dto.extensions.clone(),
            download: Some(VideoDownloadConfigModel::from(&dto.download)),
            web_search: dto.web_search.to_string(),
        }
    }
}


impl From<&ConfigDto> for ConfigModel {
    fn from(dto: &ConfigDto) -> Self {
        Self {
            threads: dto.threads as u32,
            api: Some(ConfigApiModel::from(&dto.api)),
            working_dir: dto.working_dir.to_string(),
            backup_dir: dto.backup_dir.to_string(),
            user_config_dir: dto.user_config_dir.to_string(),
            mapping_path: dto.mapping_path.to_string(),
            custom_stream_response_path: dto.custom_stream_response_path.to_string(),
            video: Some(VideoConfigModel::from(&dto.video)),
            schedules: vec![],
            log: None,
            user_access_control: dto.user_access_control,
            connect_timeout_secs: dto.connect_timeout_secs,
            sleep_timer_mins: dto.sleep_timer_mins,
            update_on_boot: dto.update_on_boot,
            config_hot_reload: dto.config_hot_reload,
            web_ui: None,
            messaging: None,
            reverse_proxy: None,
            hdhomerun: None,
            proxy: None,
            ipcheck: None,
    }
}
