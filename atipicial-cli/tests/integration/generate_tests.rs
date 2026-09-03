#[cfg(test)]
mod tests {
	use crate::integration::utils::{assert_output_contains, assert_success, CliTest};

	#[test]
	fn test_generate_list_templates() {
		let cli = CliTest::new();

		let output = cli.run_command(&["generate", "--list"]);

		assert_success(&output);
		assert_output_contains(&output, "Available Project Templates");
		assert_output_contains(&output, "Basic Atipicial dApp");
		assert_output_contains(&output, "AEP-17 Token");
	}
}
