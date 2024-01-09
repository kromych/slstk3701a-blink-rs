#[doc = "Register `MIR1_CMDREQ` reader"]
pub type R = crate::R<MIR1_CMDREQ_SPEC>;
#[doc = "Register `MIR1_CMDREQ` writer"]
pub type W = crate::W<MIR1_CMDREQ_SPEC>;
#[doc = "Field `MSGNUM` reader - Message Number"]
pub type MSGNUM_R = crate::FieldReader;
#[doc = "Field `MSGNUM` writer - Message Number"]
pub type MSGNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BUSY` reader - Busy Flag"]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    pub fn msgnum(&self) -> MSGNUM_R {
        MSGNUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    #[must_use]
    pub fn msgnum(&mut self) -> MSGNUM_W<MIR1_CMDREQ_SPEC> {
        MSGNUM_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interface Command Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_cmdreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_cmdreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIR1_CMDREQ_SPEC;
impl crate::RegisterSpec for MIR1_CMDREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir1_cmdreq::R`](R) reader structure"]
impl crate::Readable for MIR1_CMDREQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mir1_cmdreq::W`](W) writer structure"]
impl crate::Writable for MIR1_CMDREQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIR1_CMDREQ to value 0x01"]
impl crate::Resettable for MIR1_CMDREQ_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
