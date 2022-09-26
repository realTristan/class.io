import { publish } from 'gh-pages';

publish(
    './build', 
    {
        branch: 'gh-pages',
        repo: 'https://github.com/realTristan/class.io.git',
        user: {
            name: 'realTristan',
            email: 'heytristaann@gmail.com'
        },
        dotfiles: true
    },
    () => {
        console.log('Github Page: https://realtristan.github.io/class.io/');
    }
);