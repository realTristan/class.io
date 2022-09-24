<script>
	export let title;
	export let description;
</script>

<main>
	<div id="slider">
		{title}
		<div id="fade_in_text">
			<div style="margin: 10px; margin-top: 15px; color: #ff7675;">
				{description}
			</div>
		</div>
	</div>
</main>

<style>
	/* Homework Dropdown Slider Content FadeIn */
	#fade_in_text {
		animation: fadeIn 5s;
	}
	@keyframes fadeIn {
		0% { opacity: 0; }
		2% { opacity: 0; }
		20% { opacity: 1; }
	}
	/* Homework Dropdown Slider */
	#slider {
		cursor: pointer;
		padding: 10px; 
		font-weight: 700; 
		width: 500px;
		margin: 0 auto; 
		text-align: left;
		border-radius: 7px;
		margin-top: 20px;
		background-color: white;
		box-shadow: 0.5px 0.5px 8px 0.5px rgba(0, 0, 0, 0.1);
		padding: 12px 16px;
		height: 20px;
		transition:             height 500ms ease;
			-moz-transition:    height 500ms ease;
			-ms-transition:     height 500ms ease;
			-o-transition:      height 500ms ease;
			-webkit-transition: height 500ms ease;
	}
</style>