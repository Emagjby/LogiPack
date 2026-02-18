export type Role = "admin" | "employee";

// Minimal user context exposed to app shell.
export type Me = {
	name: string;
	email: string;
	role?: Role;
};

// TODO: ADD OTHER DTOS HERE TOO
