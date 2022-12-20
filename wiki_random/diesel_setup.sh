diesel setup
diesel migration generate wikiguesser
# up and down sql files 
cp migration_files/* migrations/202*/
diesel migration run
