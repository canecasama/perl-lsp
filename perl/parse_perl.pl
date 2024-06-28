#!/usr/bin/env perl

use strict;
use warnings;
use PPI;
use JSON::XS qw(encode_json);

# Get the file name from the command line arguments
my $filename = shift or die "Usage: $0 <filename>\n";

# Load the document
my $document = PPI::Document->new($filename);
die "Could not load $filename"
    unless $document;

# Function to recursively convert a PPI structure into a hash
sub convert_to_hash {
    my $node = shift;
    return unless $node;

    # Ignore comments and whitespace
    return if $node->isa('PPI::Token::Comment')
           || $node->isa('PPI::Token::Whitespace');

    my %hash = (
        type     => ref($node),
        location => [@{$node->location}[0,1]],
    );

    # Check all the child nodes
    if ($node->isa('PPI::Node')) {
        $hash{children} = [map {convert_to_hash($_)} $node->children()];
    } elsif ($node->can('content')) {
        $hash{content} = $node->content();
    }

    return \%hash;
}

# Convert the document into a hash structure and print it out in JSON format
my $hash_document = convert_to_hash($document);
print encode_json($hash_document);

1;
