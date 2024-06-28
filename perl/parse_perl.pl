#!/usr/bin/env perl

use strict;
use warnings;
use PPI;
use JSON::XS qw(encode_json);

# Get the file name from the command line arguments
my $perl_code = shift or die "Usage: $0 <perl_code>\n";

# Load the document
my $document = PPI::Document->new(\$perl_code);
die "Could not load $perl_code"
    unless $document;

# Function to recursively convert a PPI structure into a hash
sub convert_to_hash {
    my $element = shift;
    return unless $element;

    # Ignore comments and whitespace
    return if $element->isa('PPI::Token::Comment')
           || $element->isa('PPI::Token::Whitespace');

    my %hash = (
        type     => ref($element),
        location => [@{$element->location}[0,1]],
    );

    # Check all the child elements
    if ($element->isa('PPI::Node')) {
        $hash{children} = [map {convert_to_hash($_)} $element->children()];
    } elsif ($element->can('content')) {
        $hash{content} = $element->content();
    }

    return \%hash;
}

# Convert the document into a hash structure and print it out in JSON format
my $hash_document = convert_to_hash($document);
print encode_json($hash_document);

1;
