import {defineConfig} from 'vite';
import {resolve} from 'path';
export default defintConfig({
	root:'.',
	base:'./,
	build:{
		outDir:'dist',
		emptyOutDir:true
	},
	server:{
		port:5173,
		strictPort:true,
	},
	resolve:{
		alias:{
			'@':resolve(__dirnme,'./'),
		}
	}
});
