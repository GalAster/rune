use crate::compiling::v1::assemble::prelude::*;

/// Compile the body of a closure function.
impl AssembleClosure for ast::ExprClosure {
    fn assemble_closure(
        &self,
        c: &mut Compiler<'_>,
        captures: &[CompileMetaCapture],
    ) -> CompileResult<()> {
        let span = self.span();
        log::trace!("ExprClosure => {:?}", c.source.source(span));

        let count = {
            let mut patterns = Vec::new();

            for (arg, _) in self.args.as_slice() {
                match arg {
                    ast::FnArg::SelfValue(s) => {
                        return Err(CompileError::new(s, CompileErrorKind::UnsupportedSelf))
                    }
                    ast::FnArg::Pat(pat) => {
                        let offset = c.scopes.decl_anon(pat.span())?;
                        patterns.push((pat, offset));
                    }
                }
            }

            if !captures.is_empty() {
                c.asm.push(Inst::PushTuple, span);

                for capture in captures {
                    c.scopes.new_var(&capture.ident, span)?;
                }
            }

            for (pat, offset) in patterns {
                c.compile_pat_offset(pat, offset)?;
            }

            c.scopes.total_var_count(span)?
        };

        self.body.assemble(c, Needs::Value)?.apply(c)?;

        if count != 0 {
            c.asm.push(Inst::Clean { count }, span);
        }

        c.asm.push(Inst::Return, span);

        c.scopes.pop_last(span)?;
        Ok(())
    }
}

/// Compile a closure expression.
impl Assemble for ast::ExprClosure {
    fn assemble(&self, c: &mut Compiler<'_>, needs: Needs) -> CompileResult<Asm> {
        let span = self.span();
        log::trace!("ExprClosure => {:?}", c.source.source(span));

        if !needs.value() {
            c.warnings.not_used(c.source_id, span, c.context());
            return Ok(Asm::top(span));
        }

        let item = c.query.item_for(self)?;
        let hash = Hash::type_hash(&item.item);

        let meta = match c.query.query_meta(span, &item.item, Default::default())? {
            Some(meta) => meta,
            None => {
                return Err(CompileError::new(
                    span,
                    CompileErrorKind::MissingItem {
                        item: item.item.clone(),
                    },
                ))
            }
        };

        let (captures, do_move) = match &meta.kind {
            CompileMetaKind::Closure {
                captures, do_move, ..
            } => (&**captures, *do_move),
            _ => {
                return Err(CompileError::expected_meta(span, meta, "a closure"));
            }
        };

        log::trace!("captures: {} => {:?}", item.item, captures);

        if captures.is_empty() {
            // NB: if closure doesn't capture the environment it acts like a regular
            // function. No need to store and load the environment.
            c.asm.push_with_comment(
                Inst::LoadFn { hash },
                span,
                format!("closure `{}`", item.item),
            );
        } else {
            // Construct a closure environment.
            for capture in captures {
                if do_move {
                    let var = c.scopes.take_var(&capture.ident, c.source_id, span)?;
                    var.do_move(&mut c.asm, span, format!("capture `{}`", capture.ident));
                } else {
                    let var = c.scopes.get_var(&capture.ident, c.source_id, span)?;
                    var.copy(&mut c.asm, span, format!("capture `{}`", capture.ident));
                }
            }

            c.asm.push_with_comment(
                Inst::Closure {
                    hash,
                    count: captures.len(),
                },
                span,
                format!("closure `{}`", item.item),
            );
        }

        Ok(Asm::top(span))
    }
}
