<script>
	import LessonData from '../course_data.json'
	import Homework from './components/homework.svelte'
</script>

<main>
	{#each Object.entries(LessonData) as [key]}
		{@const LESSON_TITLE_SPLIT = LessonData[key]["title"].split(" |")}

		<!-- Example: Lesson #8 Factor Theorum -->
		<h1>
			<a style="color: #7c3aed;"
				rel="noopener noreferrer" target="_blank"
				href={LessonData[key]["note"]}> 
				<mark style="color: black; background: none;">
					<!-- Example: Lesson #8 -->
					{LESSON_TITLE_SPLIT[0]}
				</mark>
					<!-- Example: Factor Theorum -->
					{LESSON_TITLE_SPLIT[1]}
			</a>
		</h1>
		
		<!-- Lesson Info -->
		<div>
			<iframe 
				width="780" height="420" frameborder="0"
				src={LessonData[key]["video"]["url"]} 
				title={LessonData[key]["video"]["title"]} 
				allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen>
			</iframe>
		</div>

		<!-- Homework Title -->
		<h2>
			<a style="color: #7c3aed;"
				rel="noopener noreferrer" target="_blank"
				href={LessonData[key]["hw_full_solutions"]}>
				<mark style="color: black; background: none;">{LESSON_TITLE_SPLIT[0]}</mark> Homework
			</a>
		</h2>

		<!-- Homework Questions -->
		<Homework LessonData={LessonData[key]}/>
		<div class="seperator"></div>
	{/each}
</main>

<style>
	/* Lesson Seperator Line */
	.seperator {
		height: 2px; 
		background-color: #d4d4d8; 
		margin-top: 200px; 
		margin-bottom: 200px; 
		margin-left: 100px; 
		margin-right: 100px;
	}
	/* Main Styles */
	main {
		text-align: center;
		padding: 1em;
		margin: 0 auto;
	}
	h1 {
        text-align: center;
        color: #8b5cf6;
        font-size: 4em;
        font-weight: 700;
    }

	/* Scroll Bar */
	:root::-webkit-scrollbar {
		width: 20px;
	}
	:root::-webkit-scrollbar-track {
		background: #f1f1f1; 
	}
	:root::-webkit-scrollbar-thumb {
		background: #6366f1;
	}
	:root::-webkit-scrollbar-thumb:hover {
		background: #474af2;
	}
</style>