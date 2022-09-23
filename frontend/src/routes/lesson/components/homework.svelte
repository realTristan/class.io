<script>
    export let LessonData;
	const ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
</script>

<main>
	<!-- Homework Questions -->
	{#each LessonData["hw_questions"] as section}
		<h3 style="margin-bottom: 5px; margin-top: 10px;">{section.title}</h3>
		<div style="margin-left: 30%; margin-right: 30%;">
			<!-- Create new dropdown div for each question -->
			{#each section.questions as equation, i}
				<div id="work_div">
					<div id="slider">

						<!-- Set the equation -->
						<mark style="color: #3f3f46; background: none;">{ALPHABET[i]}) </mark>
							{equation}
						
						<!-- Set the equation solution -->
						<div id="fade_in_text">
							<div style="margin: 10px; margin-top: 15px;">
								<mark style="color: black; background: none;">&nbsp;&nbsp;Correct Answer: </mark>
								<mark style="color: #3f3f46; background: none;">&nbsp;{section.solutions[i]}</mark>
							</div>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/each}
</main>

<style>
	/* Homework Title Div */
	#work_div {
		padding: 10px;
		cursor: pointer;
		font-weight: 600;
	}
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
		margin: 0 auto; 
		text-align: left;
		color: #7c3aed;
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