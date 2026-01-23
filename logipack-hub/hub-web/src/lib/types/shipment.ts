export type Uuid = string;

export type ShipmentStatus =
	| "NEW"
	| "ACCEPTED"
	| "PROCESSED"
	| "IN_TRANSIT"
	| "DELIVERED"
	| "CANCELLED";

export type ShipmentListItem = {
	id: Uuid;
	client_id: Uuid;
	current_status: ShipmentStatus;
	current_office_id: Uuid | null;
	created_at: string;
	updated_at: string;
};

export type Client = {
	id: Uuid;
	name: string;
	email: string | null;
	phone: string | null;
};

export type Office = {
	id: Uuid;
	name: string;
	city: string;
	address: string;
};

export type ShipmentDetail = {
	id: Uuid;
	client: Client;
	current_status: ShipmentStatus;
	current_office: Office | null;
	final_destination_office: Office | null;
	created_at: string;
	updated_at: string;
};

export type TimelineItem = {
	seq: number;
	event_type: string;
	scb: string;
};

export type CreateShipmentInput = {
	client_id: Uuid;
	current_office_id: Uuid | null;
	notes: string | null;
};

export type ChangeStatusRequest = {
	to_status: ShipmentStatus;
	to_office_id: Uuid | null;
	notes: string | null;
};
