use renet::{
	// transport::{NetcodeClientTransport, NetcodeServerTransport, NetcodeTransportError},
	RenetClient, RenetServer,
};

use bevy::{app::AppExit, prelude::*};

use crate::{RenetClientPlugin, RenetReceive, RenetSend, RenetServerPlugin};

#[cfg(feature = "client")]
use renet_webtransport::prelude::{WebTransportClient, WebTransportError, WebTransportErrorSource, WebTransportOptions};
#[cfg(feature = "server")]
use renet_webtransport_server::{WebTransportConfig, WebTransportServer};

#[cfg(feature = "server")]
pub struct WebServerPlugin;

pub struct WebClientPlugin;

#[cfg(feature = "server")]
impl Plugin for WebServerPlugin {
	fn build(&self, app: &mut App) {
		// app.add_event::<NetcodeTransportError>();

		app.add_systems(
			PreUpdate,
			Self::update_system
				.in_set(RenetReceive)
				.run_if(resource_exists::<WebTransportServer>())
				.run_if(resource_exists::<RenetServer>())
				.after(RenetServerPlugin::update_system),
		);

		app.add_systems(
			PostUpdate,
			(Self::send_packets.in_set(RenetSend), Self::disconnect_on_exit)
				.run_if(resource_exists::<WebTransportServer>())
				.run_if(resource_exists::<RenetServer>()),
		);
	}
}

#[cfg(feature = "server")]
impl WebServerPlugin {
	pub fn update_system(
		mut transport: ResMut<WebTransportServer>,
		mut server: ResMut<RenetServer>,
		runtime: ResMut<bevy_tokio_tasks::TokioTasksRuntime>,
		// time: Res<Time>,
		// mut transport_errors: EventWriter<NetcodeTransportError>,
	) {
		runtime.runtime().block_on(Self::transport_update(transport, server))
		// if let Err(e) = transport.update(time.delta(), &mut server) {
		// 	transport_errors.send(e);
		// }
	}

	async fn transport_update(mut transport: ResMut<'_, WebTransportServer>, mut server: ResMut<'_, RenetServer>) {
		transport.update(&mut server)
	}

	pub fn send_packets(mut transport: ResMut<WebTransportServer>, mut server: ResMut<RenetServer>) {
		transport.send_packets(&mut server);
	}

	pub fn disconnect_on_exit(exit: EventReader<AppExit>, mut transport: ResMut<WebTransportServer>, mut server: ResMut<RenetServer>) {
		if !exit.is_empty() {
			transport.disconnect();
		}
	}
}

#[cfg(feature = "client")]
impl Plugin for WebClientPlugin {
	fn build(&self, app: &mut App) {
		// app.add_event::<NetcodeTransportError>();

		app.add_systems(
			PreUpdate,
			Self::update_system
				.in_set(RenetReceive)
				.run_if(resource_exists::<WebTransportClient>())
				.run_if(resource_exists::<RenetClient>())
				.after(RenetClientPlugin::update_system),
		);
		app.add_systems(
			PostUpdate,
			(Self::send_packets.in_set(RenetSend), Self::disconnect_on_exit)
				.run_if(resource_exists::<WebTransportClient>())
				.run_if(resource_exists::<RenetClient>()),
		);
	}
}

#[cfg(feature = "client")]
impl WebClientPlugin {
	pub fn update_system(
		mut transport: ResMut<WebTransportClient>,
		mut client: ResMut<RenetClient>,
		// time: Res<Time>,
		// mut transport_errors: EventWriter<NetcodeTransportError>,
	) {
		transport.update(&mut client)
		// if let Err(e) = transport.update(time.delta(), &mut client) {
		// 	transport_errors.send(e);
		// }
	}

	pub fn send_packets(
		mut transport: ResMut<WebTransportClient>,
		mut client: ResMut<RenetClient>,
		// mut transport_errors: EventWriter<NetcodeTransportError>,
	) {
		transport.send_packets(&mut client)
		// if let Err(e) = transport.send_packets(&mut client) {
		// 	transport_errors.send(e);
		// }
	}

	pub fn disconnect_on_exit(exit: EventReader<AppExit>, mut transport: ResMut<WebTransportClient>) {
		if !exit.is_empty() {
			transport.disconnect();
		}
	}
}
